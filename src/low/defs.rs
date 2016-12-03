/*!
    Types, constants and extern functions used in the low-level part of NWG
*/
/*
    Copyright (C) 2016  Gabriel Dubé

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

use winapi::{UINT, LRESULT};

// Custom inner message definitions

pub const NWG_CUSTOM_MIN:      UINT = 0x400;  /// Minimum custom event value
pub const NWG_PACK_USER_VALUE: UINT = 0x400;  /// Message sent when packing a user value
pub const NWG_PACK_CONTROL:    UINT = 0x401;  /// Message sent when packing a control
pub const NWG_UNPACK_CONTROL:  UINT = 0x402;  /// Message sent when removing a control from the ui
pub const NWG_UNPACK_INDIRECT: UINT = 0x403;  /// Message sent when removing the event dispatch subclass from a control
pub const NWG_CUSTOM_MAX:      UINT = 0x404;  /// Maximum custom event value

// Value returned by a window proc if the message execution failed/succeeded

pub const COMMIT_SUCCESS: LRESULT = 0;
pub const COMMIT_FAILED: LRESULT = 5555;

// Constants not included in winapi-rs

pub const ACTCTX_FLAG_RESOURCE_NAME_VALID: u32 = 0x008;
pub const ACTCTX_FLAG_SET_PROCESS_DEFAULT: u32 = 0x010;
pub const ACTCTX_FLAG_ASSEMBLY_DIRECTORY_VALID: u32 = 0x004;