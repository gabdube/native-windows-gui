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
    Persian,

    /// UmAlQura Hijri (Arabic Lunar) calendar
    UmAlQura
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
#[repr(u32)]
pub enum NegativeCurrency {
    /// Left parenthesis, monetary symbol, number, right parenthesis; for example, ($1.1)
    Mode0 = 0,	
    /// Negative sign, monetary symbol, number; for example, -$1.1
    Mode1,	
    /// Monetary symbol, negative sign, number; for example, $-1.1
    Mode2,	
    /// Monetary symbol, number, negative sign; for example, $1.1-
    Mode3,	
    /// Left parenthesis, number, monetary symbol, right parenthesis; for example, (1.1$)
    Mode4,	
    /// Negative sign, number, monetary symbol; for example, -1.1$
    Mode5,	
    /// Number, negative sign, monetary symbol; for example, 1.1-$
    Mode6,	
    /// Number, monetary symbol, negative sign; for example, 1.1$-
    Mode7,	
    /// Negative sign, number, space, monetary symbol (like #5, but with a space before the monetary symbol); for example, -1.1 $
    Mode8,	
    /// Negative sign, monetary symbol, space, number (like #1, but with a space after the monetary symbol); for example, -$ 1.1
    Mode9,	
    /// Number, space, monetary symbol, negative sign (like #7, but with a space before the monetary symbol); for example, 1.1 $-
    Mode10, 	
    /// Monetary symbol, space, number, negative sign (like #3, but with a space after the monetary symbol); for example, $ 1.1-
    Mode11,
    /// Monetary symbol, space, negative sign, number (like #2, but with a space after the monetary symbol); for example, $ -1.1 	
    Mode12, 	
    /// Number, negative sign, space, monetary symbol (like #6, but with a space before the monetary symbol); for example, 1.1- $
    Mode13, 	
    /// Left parenthesis, monetary symbol, space, number, right parenthesis (like #0, but with a space after the monetary symbol); for example, ($ 1.1)
    Mode14, 	
    /// Left parenthesis, number, space, monetary symbol, right parenthesis (like #4, but with a space before the monetary symbol); for example, (1.1 $)
    Mode15, 	
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

