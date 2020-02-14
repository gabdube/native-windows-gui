#[derive(Debug, Copy, Clone)]
pub enum MeasurementSystem {
    Metric,
    Imperial
}

#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum Calendar {
    /// Gregorian (localized) calendar
    Gregorian = 1,
    
    /// Gregorian (U.S.) calendar
    GregorianUs,

    /// Japanese Emperor Era calendar
    Japan,

    /// Taiwan Era calendar
    Taiwan,

    /// Korean Tangun Era calendar
    Korea,

    /// Hijri (Arabic Lunar) calendar
    Hijri,

    /// Thai calendar
    Thai,

    /// Hebrew (Lunar) calendar
    Hebrew,

    /// Gregorian Middle East French calendar
    GregorianMeFrench,

    /// Gregorian Arabic calendar
    GregorianArabic,

    /// Gregorian Transliterated English calendar
    GregorianXlitEnglish,
    GregorianXlitFrench,
    Julian,
    JapaneseUnisolar,
    ChineseUnisolar,
    Saka,
    LunarEtoChn,
    LunarEtoKor,
    LunarEroRokuyou,
    KoreanUnisolar,
    TaiwanUnisolar,

    /// Persian (Solar Hijri) calendar
    Persian=22,

    /// UmAlQura Hijri (Arabic Lunar) calendar
    UmAlQura=23
}

#[derive(Debug, Copy, Clone)]
pub enum NegativeNumberMode {
    /// Left parenthesis, number, right parenthesis; for example, (1.1)
    Mode0,

    /// Negative sign, number; for example, -1.1
    Mode1,

    /// Negative sign, space, number; for example, - 1.1
    Mode2,

    /// Number, negative sign; for example, 1.1-
    Mode3,

    /// Number, space, negative sign; for example, 1.1 -
    Mode4,
}

#[derive(Debug, Copy, Clone)]
pub enum PositiveCurrency {
    /// Prefix, no separation, for example, $1.1
    Mode0,

    /// Suffix, no separation, for example, 1.1$
    Mode1,

    /// Prefix, 1-character separation, for example, $ 1.1
    Mode2,

    /// Suffix, 1-character separation, for example, 1.1 $
    Mode3
}

#[derive(Debug, Copy, Clone)]
pub enum NegativeCurrency {
}

#[derive(Debug, Copy, Clone)]
pub enum FirstDayOfYear {
    /// Week containing 1/1 is the first week of the year. Note that this can be a single day, if 1/1 falls on the last day of the week.
    Mode0,

    /// First full week following 1/1 is the first week of the year.
    Mode1,

    /// First week containing at least four days is the first week of the year.
    Mode2,
}

