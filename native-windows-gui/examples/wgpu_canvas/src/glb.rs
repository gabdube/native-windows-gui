#![allow(dead_code)]

/*!
    GLB file loader
*/
use serde_json::{Value, Map};
use std::{fs, ptr, str, path::Path};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ComponentType {
    Byte,
    UByte,
    Short,
    UShort,
    Int,
    UInt,
    Float,
}

impl ComponentType {

    pub fn size(&self) -> u32 {
        use ComponentType::*;
        match self {
            Byte | UByte => 1,
            Short | UShort => 2,
            Int | UInt | Float => 4
        }
    }

    pub fn from_u64(v: u64) -> Result<ComponentType, String> {
        let c = match v {
            5120 => ComponentType::Byte,
            5121 => ComponentType::UByte,
            5122 => ComponentType::Short,
            5123 => ComponentType::UShort,
            5124 => ComponentType::Int,
            5126 => ComponentType::Float,
            _ => { return Err(format!("Unknown accessor type: {:?}", v)); }
        };

        Ok(c)
    }

}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AccessorType {
    Scalar,
    Vec2,
    Vec3,
    Vec4,
    Mat2,
    Mat3,
    Mat4
}

impl AccessorType {

    pub fn from_str(v: &str) -> Result<AccessorType, String> {
        let ty = match v {
            "SCALAR" => AccessorType::Scalar,
            "VEC2" => AccessorType::Vec2,
            "VEC3" => AccessorType::Vec3,
            "VEC4" => AccessorType::Vec3,
            "Mat2" => AccessorType::Mat2,
            "Mat3" => AccessorType::Mat3,
            "Mat4" => AccessorType::Mat4,
            _ => { return Err(format!("Unknown accessor type: {:?}", v)); }
        };

        Ok(ty)
    }

}

#[derive(Debug)]
pub struct AccessorData<'a> {
    pub component_count: u32,
    pub component_ty: ComponentType,
    pub ty: AccessorType,
    pub data: &'a [u8],
}

impl<'a> AccessorData<'a> {

    pub fn stride(&self) -> u32 {
        use AccessorType::*;

        let acc_type_size = match self.ty {
            Scalar => 1,
            Vec2 => 2,
            Vec3 => 3,
            Vec4 => 4,
            Mat2 => 4,
            Mat3 => 9,
            Mat4 => 16
        };

        self.component_ty.size() * acc_type_size
    }

}


#[derive(Debug)]
pub struct SimpleMesh {
    pub indices: u64,
    pub positions: u64,
    pub normals: Option<u64>,
    pub tex_coord: Option<u64>,
}

impl SimpleMesh {

    fn from_obj(obj: &serde_json::Map<String, Value>) -> Result<SimpleMesh, String> {
        let primitives = obj.get("primitives")
            .and_then(|p| p.as_array())
            .ok_or("SimpleMesh does not have a primitive array")?;
        
        if primitives.len() != 1 {
            return Err(format!("SimpleMesh must have on primitives group, found {}", primitives.len()));
        }

        // Here we assume the primitve group is valid
        let prim = primitives[0].as_object().unwrap();
        let attributes = prim.get("attributes").unwrap().as_object().unwrap();

        let indices = prim.get("indices").and_then(|i| i.as_u64()).unwrap();
        let positions = attributes.get("POSITION").and_then(|i| i.as_u64()).unwrap();
        let normals = attributes.get("NORMAL").and_then(|i| i.as_u64());
        let tex_coord = attributes.get("TEXCOORD_0").and_then(|i| i.as_u64());

        let mesh = SimpleMesh {
            indices,
            positions,
            normals,
            tex_coord
        };

        Ok(mesh)
    }

}

#[derive(Debug, Copy, Clone)]
pub struct NodeRef<'a> {
    name: &'a str,
    mesh: Option<usize>,
}

impl<'a> NodeRef<'a> {
    pub(crate) fn from_map(node: &'a Map<String, Value>) -> NodeRef<'a> {
        let name = node.get("name")
            .and_then(|d| d.as_str() )
            .unwrap_or("NO_NAME");

        let mesh = node.get("mesh")
            .and_then(|v| v.as_u64())
            .map(|v| v as usize);

        NodeRef {
            name,
            mesh,
        }
    }

    pub fn name(&self) -> &'a str {
        self.name
    }
}

pub struct SceneRef<'a> {
    nodes: &'a Vec<Value>,
    scene: &'a Map<String, Value>
}

impl<'a> SceneRef<'a> {
    pub fn name(&self) -> &'a str {
        self.scene.get("name")
            .and_then(|d| d.as_str() )
            .unwrap_or("NO_NAME")
    }

    pub fn nodes(&self) -> impl Iterator<Item=NodeRef<'a>> {
        let nodes = self.nodes;
        self.scene.get("nodes")
            .and_then(|v| v.as_array() )
            .and_then(move |v| Some(
                v.iter().filter_map(move |v| {
                    let index = v.as_u64().unwrap() as usize;
                    nodes.get(index)
                        .and_then(|v| v.as_object())
                        .map(|node| NodeRef::from_map(node) )
                } )
            ))
            .unwrap()
    }
}


/// Wraps the data of a "*.glb" file
pub struct GlbFile {
    bin: Box<[u8]>,
    bin_chunk_offset: isize,
    bin_chunk_size: usize,
    json: Value,
}

impl GlbFile {

    /// Read and parse a gltf binary file
    pub fn open<P: AsRef<Path>>(path: P) -> Result<GlbFile, String> {
        let path = path.as_ref();
        let content = fs::read(path)
            .map_err(|err| format!("Failed to read {:?}: {:?}", path, err) )?;

        let mut file = GlbFile {
            bin: content.into_boxed_slice(),
            bin_chunk_offset: 0,
            bin_chunk_size: 0,
            json: Value::Null,
        };

        file.parse_json()?;

        Ok(file)
    }

    /// Return a reference to the GLB json structure
    pub fn json<'a>(&'a self) -> &'a Map<String, Value> {
        match self.json.as_object() {
            Some(obj) => obj,
            None => unreachable!("Json structure is always an object"),
        }
    }

    /// Return the name of the default scene if there is one
    pub fn default_scene<'a>(&'a self) -> Option<SceneRef<'a>> {
        let json = self.json();
        let scene_id = json.get("scene")
            .and_then(|v| v.as_u64())
            .unwrap_or(u64::MAX);

        if scene_id == u64::MAX {
            return None;
        }

        let nodes = json.get("nodes").unwrap().as_array().unwrap();
        let scene = json.get("scenes").unwrap().as_array().unwrap();

        scene
            .get(scene_id as usize)
            .and_then(|v| v.as_object())
            .map(|scene| SceneRef { nodes, scene } )
    }

    /// Find a mesh by index
    pub fn simple_mesh_by_index<'a>(&'a self, index: usize) -> Result<Option<SimpleMesh>, String> {
        let meshes = self.json.get("meshes")
            .and_then(|m| m.as_array())
            .ok_or("meshes array was not found in json".to_owned())?;

        let mesh = meshes.get(index);

        match mesh.map(|m| m.as_object().unwrap() ) {
            Some(mesh_obj) => 
                SimpleMesh::from_obj(mesh_obj)
                    .map(|mesh| Some(mesh)),
            None => Ok(None),
        }
    }

    /// Find a mesh by name. The mesh cannot have more than 1 primitive group
    /// Returns an error if the meshes configuration of the GLB file is invalid
    pub fn simple_mesh_by_name<'a>(&'a self, name_ref: &str) -> Result<Option<SimpleMesh>, String> {
        let meshes = self.json.get("meshes")
            .and_then(|m| m.as_array())
            .ok_or("meshes array was not found in json".to_owned())?;

        let mesh = meshes.iter().find(|m| 
            m.as_object()
             .and_then(|mesh| mesh.get("name") )
             .and_then(|name| name.as_str() )
             .map(|name| name == name_ref )
             .unwrap_or(false)
        );

        match mesh.map(|m| m.as_object().unwrap() ) {
            Some(mesh_obj) => 
                SimpleMesh::from_obj(mesh_obj)
                    .map(|mesh| Some(mesh)),
            None => Ok(None),
        }
    }

    /// Find the mesh associated with the selected node. The mesh cannot have more than 1 primitive group
    /// Returns an error if the meshes configuration of the GLB file is invalid
    pub fn simple_mesh_by_node<'a>(&'a self, node_ref: NodeRef<'a>) -> Result<Option<SimpleMesh>, String> {
        let mesh_index = match node_ref.mesh {
            Some(index) => index,
            None => { return Ok(None); }
        };
        
        let meshes = self.json.get("meshes")
            .and_then(|m| m.as_array())
            .ok_or("meshes array was not found in json".to_owned())?;
        
        let mesh = meshes.get(mesh_index);
        match mesh.map(|m| m.as_object().unwrap() ) {
            Some(mesh_obj) => 
                SimpleMesh::from_obj(mesh_obj)
                    .map(|mesh| Some(mesh)),
            None => Ok(None),
        }
    }

    /// Return the data associated with the accessor identified by `id`
    pub fn accessor_data<'a>(&'a self, id: u64) -> Result<AccessorData<'a>, String> {
        let json = &self.json;
        
        let accessor_obj = 
            json.get("accessors")
                .and_then(|v| v.as_array())
                .and_then(|v| v.get(id as usize) as Option<&Value> )
                .and_then(|v| v.as_object())
                .ok_or(format!("Failed to read accessor with id {}", id))?;

        let component_count = accessor_obj.get("count")
            .and_then(|v| v.as_u64())
            .ok_or(format!("Failed to read count for accessor {}", id))? as u32;

        let component_ty = accessor_obj.get("componentType")
            .and_then(|v| v.as_u64())
            .map(|v| ComponentType::from_u64(v) )
            .ok_or(format!("Failed to read count for accessor {}", id))??;

        let ty = accessor_obj.get("type")
            .and_then(|v| v.as_str())
            .map(|v| AccessorType::from_str(v) )
            .ok_or(format!("Failed to read type for accessor {}", id))??;

        let data = {
            let buffer_view_id = accessor_obj.get("bufferView")
                .and_then(|v| v.as_u64())
                .ok_or(format!("Failed to read buffer view for accessor {}", id))?;

            let accessor_byte_offset = accessor_obj.get("byteOffset")
                .and_then(|v| v.as_u64())
                .unwrap_or(0);

            let buffer_view_obj = 
                json.get("bufferViews")
                    .and_then(|v| v.as_array())
                    .and_then(|v| v.get(buffer_view_id as usize) as Option<&Value> )
                    .and_then(|v| v.as_object())
                    .ok_or(format!("Failed to read buffer view with id {} from accessor id {}", buffer_view_id, id))?;
            
            let byte_length = buffer_view_obj.get("byteLength")
                .and_then(|v| v.as_u64())
                .ok_or(format!("Failed to read buffer view byte length for accessor {}", id))? as usize;

            let view_byte_offset = buffer_view_obj.get("byteOffset")
                .and_then(|v| v.as_u64())
                .unwrap_or(0);

            let combined_offset = (accessor_byte_offset + view_byte_offset) as isize;
            if combined_offset as usize > self.bin_chunk_size {
                return Err("Malformated json: mesh data points outside of the binary chunk".to_owned());
            }

            let file_offset = (combined_offset+self.bin_chunk_offset) as usize;
            &self.bin[file_offset..file_offset+byte_length]
        };

        let data = AccessorData {
            component_count,
            component_ty,
            ty,

            data
        };

        Ok(data)
    }

    fn parse_json(&mut self) -> Result<(), String> {
        const GLTF_MAGIC: u32 = 0x46546C67;
        const JSON_CHUNK: u32 = 0x4E4F534A;
        const BIN_CHUNK: u32 = 0x004E4942;

        let src = self.bin.as_ptr();
        let src_size = self.bin.len();
        let json_size;
        let mut offset = 0;

        if src_size < 24 {
            let msg = format!("Glb buffer is not big enough: {} bytes", self.bin.len());
            return Err(msg);
        }

        // Header chunk parsing
        unsafe {
            let magic = ptr::read::<u32>(src.offset(offset) as _);
            if magic != GLTF_MAGIC {
                let msg = format!("Invalid magic number. Expected 0x{:X}, got 0x{:X}", GLTF_MAGIC, magic);
                return Err(msg);
            }

            offset += 4;
            let version = ptr::read::<u32>(src.offset(offset) as _);
            if version != 2 {
                let msg = format!("Invalid gltf version. Expected 2, got {}", version);
                return Err(msg);
            }

            offset += 4;
            let length = ptr::read::<u32>(src.offset(offset) as _) as usize;
            if length != src_size {
                let msg = format!("Size mismatch between buffer & gltf header: {}(buffer) VS {}(file)", src_size, length);
                return Err(msg);
            }
        }

        // JSON chunk parsing
        unsafe {
            offset += 4;
            json_size = ptr::read::<u32>(src.offset(offset) as _) as usize;

            offset += 4;
            let ty = ptr::read::<u32>(src.offset(offset) as _);
            if ty != JSON_CHUNK {
                let msg = format!("GLB first chunk type should be JSON, got 0x{:X}", ty);
                return Err(msg);
            }

            offset += 4;
            let json: Value = {
                if (offset as usize) + json_size > src_size {
                    let msg = format!("JSON chunk size exceed buffer size. Reported chunk size: {}, total buffer size: {}", json_size, src_size);
                    return Err(msg);
                }

                let o = offset as usize;
                let json_str = str::from_utf8(&self.bin[o..(o+json_size)])
                    .map_err(|e| format!("Failed to decode json chunk: {:?}", e) )?;
                
                serde_json::from_str(json_str)
                    .map_err(|e| format!("Failed to parse json chunk: {:?}", e) )?
            };

            self.json = json;
        }

        // BIN chunk
        unsafe {
            offset += json_size as isize;
            let size = ptr::read::<u32>(src.offset(offset) as _) as usize;

            offset += 4;
            let ty = ptr::read::<u32>(src.offset(offset) as _) as u32;
            if ty != BIN_CHUNK {
                let msg = format!("GLB second chunk type should be BIN, got 0x{:X}", ty);
                return Err(msg);
            }

            offset += 4;

            if (offset as usize) + size > src_size {
                let msg = format!("BIN chunk size exceed buffer size. Reported chunk size: {}, total buffer size: {}", size, src_size);
                return Err(msg);
            }

            self.bin_chunk_size = size;
            self.bin_chunk_offset = offset;
        }

        Ok(())
    }

}


#[test]
fn test_glb() {
    let cube = GlbFile::open("./assets/models/BoxTextured.glb").expect("Failed to load file");
    let json = cube.json();

    let meshes = json.get("meshes").expect("No meshes in glb file");
    assert!(meshes.is_array(), "Meshes should be array");

    let meshes = meshes.as_array().unwrap();
    assert!(meshes.len() == 1, "File should only have 1 mesh");

    let no_mesh = cube.simple_mesh_by_name("test");
    assert!(no_mesh.is_ok(), "Failing to find a mesh should not raise an error");
    assert!(no_mesh.unwrap().is_none(), "no_mesh should be none");

    let cube_mesh = cube.simple_mesh_by_name("Mesh");
    assert!(cube_mesh.is_ok(), "mesh_by_name should be ok");
    assert!(cube_mesh.unwrap().is_some(), "cube_mesh should be some");

    let cube_mesh = meshes.get(0).expect("Expected a mesh with id 0");
    match cube_mesh.get("name") {
        Some(name) => {
            assert!(name.is_string(), "Name should be string");
            assert!(name.as_str().unwrap() == "Mesh", "Name should be \"Mesh\"");
        },
        None => panic!("Mesh should have a name")
    };

    let primitives = match cube_mesh.get("primitives") {
        Some(primitives) => {
            assert!(primitives.is_array(), "Primitive should be array");

            let primitives = primitives.as_array().unwrap();
            assert!(primitives.len() == 1, "Should only have 1 primitive");

            primitives.get(0).unwrap()
        },
        None => panic!("Mesh should have primitives")
    };

    let index_accessor = primitives.get("indices").unwrap().as_u64().unwrap();
    let index_data = cube.accessor_data(index_accessor).expect("Failed to fetch accessor data");

    assert!(index_data.component_count == 36);
    assert!(index_data.component_ty == ComponentType::UShort);
    assert!(index_data.ty == AccessorType::Scalar);
    assert!(index_data.data.len() == 36*2);
}
