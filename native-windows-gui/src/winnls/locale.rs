use winapi::um::winnls::{GetLocaleInfoEx, GetUserDefaultLocaleName, GetSystemDefaultLocaleName, LCTYPE};
use winapi::um::winnt::{LOCALE_NAME_MAX_LENGTH};
use crate::win32::base_helper::{to_utf16, from_utf16};
use crate::NwgError;
use std::ptr;


/**
    Represent a Windows locale.

    Use `Locale::user` to fetch the current user local or `Local::system` to fetch the system default locale.
    Using the first one is recommended.
*/
#[derive(Clone)]
pub struct Locale {
    name: String,
    name_buffer: Vec<u16>
}

impl Locale {

    /// Create a new local from a locale name.
    pub fn new(name: String) -> Result<Locale, NwgError> {
        let name_buffer = to_utf16(&name);
        match Locale::locale_valid(&name_buffer) {
            true => Ok(Locale { name, name_buffer }),
            false => Err(NwgError::bad_locale("Locale name is not valid"))
        }
    }

    /// Return the current user locale
    pub fn user() -> Locale {
        let mut name_buffer: Vec<u16> = Vec::with_capacity(LOCALE_NAME_MAX_LENGTH);
        unsafe {
            name_buffer.set_len(LOCALE_NAME_MAX_LENGTH);
            GetUserDefaultLocaleName(name_buffer.as_mut_ptr(), LOCALE_NAME_MAX_LENGTH as i32);
        }

        Locale {
            name: from_utf16(&name_buffer),
            name_buffer
        }
    }

    /// Return the current system locale
    pub fn system() -> Locale {
        let mut name_buffer: Vec<u16> = Vec::with_capacity(LOCALE_NAME_MAX_LENGTH);
        unsafe {
            name_buffer.set_len(LOCALE_NAME_MAX_LENGTH);
            GetSystemDefaultLocaleName(name_buffer.as_mut_ptr(), LOCALE_NAME_MAX_LENGTH as i32);
        }

        Locale {
            name: from_utf16(&name_buffer),
            name_buffer
        }
    }

    /// Return the name of the locale.
    pub fn name(&self) -> &str {
        &self.name
    }

    /**
        Return the localized month name. See `month_name_abv` for the abbreviated version
        
        Parameters:
            month_index: The month index. 1(January) to 12 (December) or 13 (if it exists).

        Panics:
            This function will panic if month index in not in the 1-13 range.
    */
    pub fn month_name(&self, month_index: u32) -> String {
        match month_index {
            1 => self.get_locale_info_string(0x00000038),
            2 => self.get_locale_info_string(0x00000039),
            3 => self.get_locale_info_string(0x0000003A),
            4 => self.get_locale_info_string(0x0000003B),
            5 => self.get_locale_info_string(0x0000003C),
            6 => self.get_locale_info_string(0x0000003D),
            7 => self.get_locale_info_string(0x0000003E),
            8 => self.get_locale_info_string(0x0000003F),
            9 => self.get_locale_info_string(0x00000040),
            10 => self.get_locale_info_string(0x00000041),
            11 => self.get_locale_info_string(0x00000042),
            12 => self.get_locale_info_string(0x00000043),
            13 => self.get_locale_info_string(0x0000100E),
            x => panic!("{} is not a valid month index", x)
        }
    }

    /**
        Return the localized month name in an abbreviated version. See `month_name` for the full version
        
        Parameters:
            month_index: The month index. 1(January) to 12 (December) or 13 (if it exists).

        Panics:
            This function will panic if month index in not in the 1-13 range.
    */
    pub fn month_name_abv(&self, month_index: u32) -> String {
        match month_index {
            1 => self.get_locale_info_string(0x00000044),
            2 => self.get_locale_info_string(0x00000045),
            3 => self.get_locale_info_string(0x00000046),
            4 => self.get_locale_info_string(0x00000047),
            5 => self.get_locale_info_string(0x00000048),
            6 => self.get_locale_info_string(0x00000049),
            7 => self.get_locale_info_string(0x0000004A),
            8 => self.get_locale_info_string(0x0000004B),
            9 => self.get_locale_info_string(0x0000004C),
            10 => self.get_locale_info_string(0x0000004D),
            11 => self.get_locale_info_string(0x0000004E),
            12 => self.get_locale_info_string(0x0000004F),
            13 => self.get_locale_info_string(0x0000100F),
            x => panic!("{} is not a valid month index", x)
        }
    }

    fn get_locale_info_string(&self, info: LCTYPE, ) -> String {
        unsafe {
            let buffer_size = GetLocaleInfoEx(self.name_buffer.as_ptr(), info, ptr::null_mut(), 0) as usize;
            let mut buffer: Vec<u16> = Vec::with_capacity(buffer_size);
            buffer.set_len(buffer_size);

            GetLocaleInfoEx(self.name_buffer.as_ptr(), info, buffer.as_mut_ptr(), buffer_size as i32);

            from_utf16(&buffer)
        }
    }

    fn locale_valid(buffer: &[u16]) -> bool {
        let result = unsafe { GetLocaleInfoEx(buffer.as_ptr(), 0x00000038, ptr::null_mut(), 0) };
        result != 0
    }

}

use std::fmt;

impl fmt::Debug for Locale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Locale {{ name: {}, }}", self.name)
    }
}

impl fmt::Display for Locale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
