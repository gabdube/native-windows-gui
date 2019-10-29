/*!
    Identifier for built-in system resoucres
*/


/**
    List of built-in system images identifiers. To use with the `Image` resource.
*/
#[derive(Clone, PartialEq, Debug)]
pub enum OemImage {
    Cursor(OemCursor),
    Bitmap(OemBitmap),
    Icon(OemIcon),
}

/**
    List of system cursors. To use with the `Image` resource.
*/
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum OemCursor {
    Normal = 32512,
    IBeam = 32513,
    Wait = 32514,
    Cross = 32515,
    Up = 32516,
    Size = 32640,
    Icon = 32641,
    SizeNWSE = 32642,
    SizeNESW = 32643,
    SizeWE = 32644,
    SizeNS = 32645,
    SizeALL = 32646,
    No = 32648,
    AppStarting = 32650
}

/**
    List of system bitmaps. To use with the `Image` resource.
*/
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum OemBitmap {
    BtnCorners = 32758,
    BtSize = 32761,
    Check = 32760,
    CheckBoxes = 32759,
    Close = 32754,
    Combo = 32738,
    DnArrow = 32752,
    DnArrowD = 32742,
    DnArrowI = 32736,
    LfArrow = 32750,
    LfArrowI = 32734,
    LfrrowD = 32740,
    MnArrow = 32739,
    OldCLOSE = 32767,
    OldDnArrow = 32764,
    OldLfArrow = 32762,
    OldReduce = 32757,
    OldRestore = 32755,
    OldRgArrow = 32763,
    OldUpArrow = 32765,
    OldZoom = 32756,
    Reduce = 32749,
    Reduced = 32746,
    Restore = 32747,
    Restored = 32744,
    RgArrow = 32751,
    RgArrowD = 32741,
    RgArrowI = 32735,
    Size = 32766,
    UpArrow = 32753,
    UpArrowD = 32743,
    UpArrowI = 32737,
    Zoom = 32748,
    ZoomD = 32745,
}

/**
    List of system icons. To use with the `Image` resource.
*/
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum OemIcon {
    Sample = 32512,
    Ques = 32514,
    WinLogo = 32517,
    Warning = 32515,
    Error = 32513,
    Information = 32516
}
