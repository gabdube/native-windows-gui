use winapi::um::winnls::{GetLocaleInfoEx, GetUserDefaultLocaleName, GetSystemDefaultLocaleName, LCTYPE};
use winapi::um::winnt::{LOCALE_NAME_MAX_LENGTH, LPWSTR};
use super::*;
use crate::win32::base_helper::{to_utf16, from_utf16};
use crate::NwgError;
use std::{mem, ptr};


/**
Represent a Windows locale. Can be used to fetch a lot of information regarding the locale.

Use `Locale::user` to fetch the current user local or `Local::system` to fetch the system default locale. Using the first one is recommended.

```rust
use native_windows_gui as nwg;

let fr_locale = nwg::Locale::from_str("fr");
let en_us_locale = nwg::Locale::from_str("en-US");
let user_locale = nwg::Locale::user();
let locales: Vec<String> = nwg::Locale::all();

user_locale.display_name();
```
*/
#[derive(Clone)]
pub struct Locale {
    name: String,
    name_buffer: Vec<u16>
}

impl Locale {

    /// Create a new local from a locale name. If you have a str, use `from_str` instead.
    pub fn new(name: String) -> Result<Locale, NwgError> {
        let name_buffer = to_utf16(&name);
        match Locale::locale_valid(&name_buffer) {
            true => Ok(Locale { name, name_buffer }),
            false => Err(NwgError::bad_locale("Locale name is not valid"))
        }
    }

    /// Create a new local from a locale name. If you have a String, use `new` instead.
    pub fn from_str<'a>(name: &'a str) -> Result<Locale, NwgError> {
        let name_buffer = to_utf16(name);
        match Locale::locale_valid(&name_buffer) {
            true => Ok(Locale { name: name.to_string(), name_buffer }),
            false => Err(NwgError::bad_locale("Locale name is not valid"))
        }
    }

    /// Return the identifier (ex: en-US) of every supported locales.
    pub fn all() -> Vec<String> {
        use winapi::um::winnls::EnumSystemLocalesEx;
        use winapi::shared::minwindef::{DWORD, BOOL, LPARAM};
        use crate::win32::base_helper::from_wide_ptr;

        unsafe extern "system" fn enum_locales(locale: LPWSTR, _flags: DWORD, p: LPARAM) -> BOOL {
            let locales: *mut Vec<String> = p as *mut Vec<String>;
            (&mut *locales).push(from_wide_ptr(locale, None));
            1
        }

        unsafe {
            let mut locales: Vec<String> = Vec::with_capacity(10);
            EnumSystemLocalesEx(Some(enum_locales), 1, &mut locales as *mut Vec<String> as LPARAM, ptr::null_mut());
            locales
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

    /// Localized name of locale, eg "German (Germany)" in UI language
    pub fn display_name(&self) -> String {
        self.get_locale_info_string(0x00000002)
    }

    /// Display name (language + country/region usually) in English, eg "German (Germany)"
    pub fn english_display_name(&self) -> String {
        self.get_locale_info_string(0x00000072)
    }

    /// Display name in native locale language, eg "Deutsch (Deutschland)
    pub fn native_display_name(&self) -> String {
        self.get_locale_info_string(0x00000072)
    }

    /// Localized name of country/region, eg "Germany" in UI language
    pub fn country_name(&self) -> String {
        self.get_locale_info_string(0x00000006)
    }

    /// English name of country/region, eg "Germany"
    pub fn english_country_name(&self) -> String {
        self.get_locale_info_string(0x00001002)
    }

    /// Native name of country/region, eg "Deutschland"
    pub fn native_country_name(&self) -> String {
        self.get_locale_info_string(0x00000008)
    }

    /// country/region dialing code, example: en-US and en-CA return 1.
    pub fn dialing_code(&self) -> i32 {
        self.get_locale_info_int(0x00000005)
    }

    /// list item separator, eg "," for "1,2,3,4"
    pub fn list_separator(&self) -> String {
        self.get_locale_info_string(0x0000000C)
    }

    /// Returns the measurement system (metric or imperial)
    pub fn measurement_system(&self) -> MeasurementSystem {
        match self.get_locale_info_int(0x0000000D) { 
            0 => MeasurementSystem::Metric,
            _ => MeasurementSystem::Imperial,
        }
    }

    /// Returns the decimal separator, eg "." for 1,234.00
    pub fn decimal_separator(&self) -> String {
        self.get_locale_info_string(0x0000000E)
    }

    /// Returns the thousand separator, eg "," for 1,234.00
    pub fn thousand_separator(&self) -> String {
        self.get_locale_info_string(0x0000000F)
    }

    /// Returns the digit grouping, eg "3;0" for 1,000,000
    pub fn digit_grouping(&self) -> String {
        self.get_locale_info_string(0x00000010)
    }

    /// Returns the number of fractional digits eg 2 for 1.00
    pub fn fractional_digit(&self) -> i32 {
        self.get_locale_info_int(0x00000011)
    }

    /// Returns the number of leading zeros for decimal, 0 for .97, 1 for 0.97
    pub fn leading_zeros(&self) -> i32 {
        self.get_locale_info_int(0x00000012)
    }

    /// Returns the negative number mode. See the documentation of NegativeNumberMode
    pub fn negative_number_mode(&self) -> NegativeNumberMode {
        match self.get_locale_info_int(0x00001010) {
            0 => NegativeNumberMode::Mode0,
            1 => NegativeNumberMode::Mode1,
            2 => NegativeNumberMode::Mode2,
            3 => NegativeNumberMode::Mode3,
            4 => NegativeNumberMode::Mode4,
            _ => NegativeNumberMode::Mode1,
        }
    }

    /// Returns native digits for 0-9, eg "0123456789"
    pub fn native_digits(&self) -> String {
        self.get_locale_info_string(0x00000013)
    }

    /// Returns the local monetary symbol, eg "$"
    pub fn currency_symbol(&self) -> String {
        self.get_locale_info_string(0x00000014)
    }

    /// Returns the intl monetary symbol, eg "USD"
    pub fn intl_monetary_symbol(&self) -> String {
        self.get_locale_info_string(0x00000015)
    }

    /// Returns the monetary decimal separator, eg "." for 1,234.00
    pub fn monetary_decimal_separator(&self) -> String {
        self.get_locale_info_string(0x00000016)
    }

    /// Returns the monetary thousand separator, eg "," for 1,234.00
    pub fn monetary_thousand_separator(&self) -> String {
        self.get_locale_info_string(0x00000017)
    }

    /// Returns the monetary digit grouping, eg "3;0" for 1,000,000
    pub fn monetary_digit_grouping(&self) -> String {
        self.get_locale_info_string(0x00000018)
    }

    /// Returns the number local monetary digits eg 2 for $1.00
    pub fn monetary_fractional_digit(&self) -> i32 {
        self.get_locale_info_int(0x00000019)
    }

    /// Returns the positive currency mode. See PositiveCurrency
    pub fn currency_mode(&self) -> PositiveCurrency {
        match self.get_locale_info_int(0x0000001B) {
            0 => PositiveCurrency::Mode0,
            1 => PositiveCurrency::Mode1,
            2 => PositiveCurrency::Mode2,
            3 => PositiveCurrency::Mode3,
            _ => PositiveCurrency::Mode1,
        }
    }

    /// Returns the negative positive currency mode. See NegativeCurrency
    pub fn negative_currency_mode(&self) -> NegativeCurrency {
        let id = self.get_locale_info_int(0x00001009) as u32;
        match id <= 15 {
            true => unsafe { mem::transmute(id) },
            false => NegativeCurrency::Mode1
        }
    }

    /// Returns the short date format string, eg "MM/dd/yyyy"
    pub fn short_date(&self) -> String {
        self.get_locale_info_string(0x0000001F)
    }

    /// Returns the long date format string, eg "dddd, MMMM dd, yyyy"
    pub fn long_date(&self) -> String {
        self.get_locale_info_string(0x00000020)
    }

    /// Returns the time format string, eg "HH:mm:ss"
    pub fn time(&self) -> String {
        self.get_locale_info_string(0x00001003)
    }

    /// Returns the AM designator, eg "AM"
    pub fn am(&self) -> String {
        self.get_locale_info_string(0x00000028)
    }

    /// Returns the PM designator, eg "PM"
    pub fn pm(&self) -> String {
        self.get_locale_info_string(0x00000029)
    }

    /// Returns the type of calendar. Ex: Calendar::Gregorian
    pub fn calendar(&self) -> Calendar {
        let id = self.get_locale_info_int(0x00001009) as u32;
        match id <= 23 {
            true => unsafe { mem::transmute(id) },
            false => Calendar::Gregorian
        }
    }

    /// Returns the type of calendar with a bit more precision. Ex: Calendar::GregorianUs
    pub fn calendar2(&self) -> Calendar {
        let id = self.get_locale_info_int(0x0000100B) as u32;
        match id <= 23 {
            true => unsafe { mem::transmute(id) },
            false => Calendar::Gregorian
        }
    }

    /// Returns the first day of week specifier, 0-6, 0=Monday, 6=Sunday
    pub fn first_day_of_week(&self) -> i32 {
        self.get_locale_info_int(0x0000100C)
    }

    /// Returns the first day of year specifier. See FirstDayOfYear
    pub fn first_day_of_year(&self) -> FirstDayOfYear {
        match self.get_locale_info_int(0x0000100D) {
            0 => FirstDayOfYear::Mode0,
            1 => FirstDayOfYear::Mode1,
            2 => FirstDayOfYear::Mode2,
            _ => FirstDayOfYear::Mode0,
        }
    }

    /// ISO abbreviated language name, eg "en"
    pub fn iso_lang_name(&self) -> String {
        self.get_locale_info_string(0x00000059)
    }

    /// ISO abbreviated country/region name, eg "US"
    pub fn iso_country_name(&self) -> String {
        self.get_locale_info_string(0x0000005A)
    }

    /// Returns the english name of currency, eg "Euro"
    pub fn currency_name(&self) -> String {
        self.get_locale_info_string(0x00001007)
    }

    /// Returns the native name of currency, eg "euro"
    pub fn native_currency_name(&self) -> String {
        self.get_locale_info_string(0x00001008)
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

    /**
        Return the localized day name. See `day_name_abv` for the abbreviated version
        
        Parameters:
            day_index: The month index. 1(Monday) to 7 (Sunday)

        Panics:
            This function will panic if month index in not in the 1-7 range.
    */
    pub fn day_name(&self, day_index: u32) -> String {
        match day_index {
            1 => self.get_locale_info_string(0x0000002A),
            2 => self.get_locale_info_string(0x0000002B),
            3 => self.get_locale_info_string(0x0000002C),
            4 => self.get_locale_info_string(0x0000002D),
            5 => self.get_locale_info_string(0x0000002E),
            6 => self.get_locale_info_string(0x0000002F),
            7 => self.get_locale_info_string(0x00000030),
            x => panic!("{} is not a valid day index", x)
        }
    }

    /**
        Return the localized day name in an abbreviated version. See `day_name` for the full version
        
        Parameters:
            day_index: The month index. 1(Monday) to 7 (Sunday)

        Panics:
            This function will panic if month index in not in the 1-7 range.
    */
    pub fn day_name_abv(&self, day_index: u32) -> String {
        match day_index {
            1 => self.get_locale_info_string(0x00000031),
            2 => self.get_locale_info_string(0x00000032),
            3 => self.get_locale_info_string(0x00000033),
            4 => self.get_locale_info_string(0x00000034),
            5 => self.get_locale_info_string(0x00000035),
            6 => self.get_locale_info_string(0x00000036),
            7 => self.get_locale_info_string(0x00000037),
            x => panic!("{} is not a valid day index", x)
        }
    }

    fn get_locale_info_string(&self, info: LCTYPE) -> String {
        unsafe {
            let buffer_size = GetLocaleInfoEx(self.name_buffer.as_ptr(), info, ptr::null_mut(), 0) as usize;
            let mut buffer: Vec<u16> = Vec::with_capacity(buffer_size);
            buffer.set_len(buffer_size);

            GetLocaleInfoEx(self.name_buffer.as_ptr(), info, buffer.as_mut_ptr(), buffer_size as i32);

            from_utf16(&buffer)
        }
    }

    fn get_locale_info_int(&self, info: LCTYPE) -> i32 {
        let mut out = 0i32;
        let return_number = 0x20000000;

        unsafe {
            let out_ptr = &mut out as *mut i32 as *mut u16;
            GetLocaleInfoEx(self.name_buffer.as_ptr(), info | return_number, out_ptr, mem::size_of::<i32>() as i32);
        }
        

        out
    }

    fn locale_valid(buffer: &[u16]) -> bool {
        use winapi::um::winnls::IsValidLocaleName;
        unsafe { IsValidLocaleName(buffer.as_ptr()) != 0 }
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
