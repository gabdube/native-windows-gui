#[cfg(not(feature = "high-dpi"))]
#[deprecated(note = "Specifying the default process DPI awareness via API is not recommended. Use the '<dpiAware>true</dpiAware>' setting in the application manifest. https://docs.microsoft.com/ru-ru/windows/win32/hidpi/setting-the-default-dpi-awareness-for-a-process")]
pub unsafe fn set_dpi_awareness() {
}

#[cfg(feature = "high-dpi")]
#[deprecated(note = "Specifying the default process DPI awareness via API is not recommended. Use the '<dpiAware>true</dpiAware>' setting in the application manifest. https://docs.microsoft.com/ru-ru/windows/win32/hidpi/setting-the-default-dpi-awareness-for-a-process")]
pub unsafe fn set_dpi_awareness() {
    use winapi::um::winuser::SetProcessDPIAware;
    SetProcessDPIAware();
}

#[cfg(not(feature = "high-dpi"))]
pub fn scale_factor() -> f64 {
    return 1.0;
}

#[cfg(feature = "high-dpi")]
pub fn scale_factor() -> f64 {
    use winapi::um::winuser::USER_DEFAULT_SCREEN_DPI;
    let dpi = unsafe { dpi() };
    f64::from(dpi) / f64::from(USER_DEFAULT_SCREEN_DPI)
}

#[cfg(not(feature = "high-dpi"))]
pub unsafe fn logical_to_physical(x: i32, y: i32) -> (i32, i32) {
    (x, y)
}

#[cfg(feature = "high-dpi")]
pub unsafe fn logical_to_physical(x: i32, y: i32) -> (i32, i32) {
    use muldiv::MulDiv;
    use winapi::um::winuser::USER_DEFAULT_SCREEN_DPI;
    let dpi = dpi();
    let x = x.mul_div_round(dpi, USER_DEFAULT_SCREEN_DPI).unwrap_or(x);
    let y = y.mul_div_round(dpi, USER_DEFAULT_SCREEN_DPI).unwrap_or(y);
    (x, y)
}

#[cfg(not(feature = "high-dpi"))]
pub unsafe fn physical_to_logical(x: i32, y: i32) -> (i32, i32) {
    (x, y)
}

#[cfg(feature = "high-dpi")]
pub unsafe fn physical_to_logical(x: i32, y: i32) -> (i32, i32) {
    use muldiv::MulDiv;
    use winapi::um::winuser::USER_DEFAULT_SCREEN_DPI;
    let dpi = dpi();
    let x = x.mul_div_round(USER_DEFAULT_SCREEN_DPI, dpi).unwrap_or(x);
    let y = y.mul_div_round(USER_DEFAULT_SCREEN_DPI, dpi).unwrap_or(y);
    (x, y)
}

pub unsafe fn dpi() -> i32 {
    use winapi::um::winuser::GetDC;
    use winapi::um::wingdi::GetDeviceCaps;
    use winapi::um::wingdi::LOGPIXELSX;
    let screen = GetDC(std::ptr::null_mut());
    let dpi = GetDeviceCaps(screen, LOGPIXELSX);
    dpi
}
