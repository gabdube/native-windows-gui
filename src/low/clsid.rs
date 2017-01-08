use winapi::GUID;

#[macro_export]
macro_rules! define_guid {
    ($n1:ident, $d1:expr, $d2:expr, $d3:expr, $d4:expr) => (

        #[inline(always)]
        pub fn $n1() -> GUID {
            GUID {
                Data1: $d1,
                Data2: $d2,
                Data3: $d3,
                Data4: $d4
            }
        }
    
    )
}

define_guid!(CLSID_FileOpenDialog, 3692845724, 59530, 19934, [165, 161, 96, 248, 42, 32, 174, 247]);
define_guid!(UUIDOF_IFileDialog, 1123569974, 56190, 17308, [133, 241, 228, 7, 93, 19, 95, 200]);