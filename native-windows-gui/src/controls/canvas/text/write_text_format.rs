/*!
Wrapper over a IDWriteTextFormat interface. 
The TextFormat interface describes the font and paragraph properties used to format text, and it describes locale information.

A `WriteTextFormat` is generated from `WriteFactory::CreateTextFormat`.

Winapi documentation: https://docs.microsoft.com/en-us/windows/win32/api/dwrite/nn-dwrite-idwritetextformat
*/

use winapi::um::dwrite::IDWriteTextFormat;

/// See module level documentation
pub struct WriteTextFormat {
    handle: *mut IDWriteTextFormat
}
