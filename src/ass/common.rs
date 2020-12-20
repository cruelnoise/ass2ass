use super::AssParseError::{
    self, BadAlignment, BadBorderStyle, BadColourCode, BadEncoding, BadLineFormat, BadTimeCode,
    BadWrapStyle, BadYCbCrMatrix,
};
use lazy_static::lazy_static;
use regex::Regex;
use std::{fmt, str::FromStr, time::Duration};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WrapStyle {
    Zero,
    One,
    Two,
    Three,
}
impl fmt::Display for WrapStyle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::WrapStyle::*;
        write!(
            f,
            "{}",
            match *self {
                Zero => "0",
                One => "1",
                Two => "2",
                Three => "3",
            }
        )
    }
}
impl FromStr for WrapStyle {
    type Err = AssParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::WrapStyle::*;
        Ok(match s {
            "0" => Zero,
            "1" => One,
            "2" => Two,
            "3" => Three,
            _ => return Err(BadWrapStyle),
        })
    }
}
impl Default for WrapStyle {
    fn default() -> Self {
        WrapStyle::Zero
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum YCbCrMatrix {
    Bt601TV,
    Bt601PC,
    Bt709TV,
    Bt709PC,
    Smpte240mTV,
    Smpte240mPC,
    FccTV,
    FccPC,
}
impl fmt::Display for YCbCrMatrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::YCbCrMatrix::*;
        write!(
            f,
            "{}",
            match *self {
                Bt601TV => "TV.601",
                Bt601PC => "PC.601",
                Bt709TV => "TV.709",
                Bt709PC => "PC.709",
                Smpte240mTV => "TV.240M",
                Smpte240mPC => "PC.240M",
                FccTV => "TV.FCC",
                FccPC => "PC.FCC",
            }
        )
    }
}
impl FromStr for YCbCrMatrix {
    type Err = AssParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::YCbCrMatrix::*;
        Ok(match s.to_lowercase().as_str() {
            "tv.601" => Bt601TV,
            "pc.601" => Bt601PC,
            "tv.709" => Bt709TV,
            "pc.709" => Bt709PC,
            "tv.240m" => Smpte240mTV,
            "pc.240m" => Smpte240mPC,
            "tv.fcc" => FccTV,
            "pc.fcc" => FccPC,
            _ => return Err(BadYCbCrMatrix),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BorderStyle {
    One,
    Three,
    Four,
}
impl fmt::Display for BorderStyle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::BorderStyle::*;
        write!(
            f,
            "{}",
            match *self {
                One => "1",
                Three => "3",
                Four => "4",
            }
        )
    }
}
impl FromStr for BorderStyle {
    type Err = AssParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::BorderStyle::*;
        Ok(match s {
            "1" => One,
            "3" => Three,
            "4" => Four,
            _ => return Err(BadBorderStyle),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Alignment {
    BottomLeft,
    BottomCenter,
    BottomRight,
    Left,
    Center,
    Right,
    TopLeft,
    TopCenter,
    TopRight,
}
impl fmt::Display for Alignment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Alignment::*;
        write!(
            f,
            "{}",
            match *self {
                BottomLeft => "1",
                BottomCenter => "2",
                BottomRight => "3",
                Left => "4",
                Center => "5",
                Right => "6",
                TopLeft => "7",
                TopCenter => "8",
                TopRight => "9",
            }
        )
    }
}
impl FromStr for Alignment {
    type Err = AssParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::Alignment::*;
        Ok(match s {
            "1" => BottomLeft,
            "2" => BottomCenter,
            "3" => BottomRight,
            "4" => Left,
            "5" => Center,
            "6" => Right,
            "7" => TopLeft,
            "8" => TopCenter,
            "9" => TopRight,
            _ => return Err(BadAlignment),
        })
    }
}
impl Default for Alignment {
    fn default() -> Self {
        Alignment::BottomCenter
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Encoding {
    Ansi,
    Default,
    Symbol,
    Mac,
    ShiftJIS,
    Hangeul,
    Johab,
    Gb2312,
    ChineseBIG5,
    Greek,
    Turkish,
    Vietnamese,
    Hebrew,
    Arabic,
    Baltic,
    Russian,
    Thai,
    EastEuropean,
    Oem,
}
impl fmt::Display for Encoding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Encoding::*;
        write!(
            f,
            "{}",
            match *self {
                Ansi => "0",
                Default => "1",
                Symbol => "2",
                Mac => "77",
                ShiftJIS => "128",
                Hangeul => "129",
                Johab => "130",
                Gb2312 => "134",
                ChineseBIG5 => "136",
                Greek => "161",
                Turkish => "162",
                Vietnamese => "163",
                Hebrew => "177",
                Arabic => "178",
                Baltic => "186",
                Russian => "204",
                Thai => "222",
                EastEuropean => "238",
                Oem => "255",
            }
        )
    }
}
impl FromStr for Encoding {
    type Err = AssParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::Encoding::*;
        Ok(match s {
            "0" => Ansi,
            "1" => Default,
            "2" => Symbol,
            "77" => Mac,
            "128" => ShiftJIS,
            "129" => Hangeul,
            "130" => Johab,
            "134" => Gb2312,
            "136" => ChineseBIG5,
            "161" => Greek,
            "162" => Turkish,
            "163" => Vietnamese,
            "177" => Hebrew,
            "178" => Arabic,
            "186" => Baltic,
            "204" => Russian,
            "222" => Thai,
            "238" => EastEuropean,
            "255" => Oem,
            _ => return Err(BadEncoding),
        })
    }
}
impl Default for Encoding {
    fn default() -> Self {
        Encoding::Default
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct ABGR(u8, u8, u8, u8);
impl fmt::Display for ABGR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "&H{:02X}{:02X}{:02X}{:02X}",
            self.0, self.1, self.2, self.3
        )
    }
}
impl FromStr for ABGR {
    type Err = AssParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 10 && &s[..2] == "&H" {
            match u32::from_str_radix(&s[2..], 16) {
                Ok(i) => Ok(ABGR::from(i)),
                Err(_) => Err(BadColourCode),
            }
        } else {
            Err(BadColourCode)
        }
    }
}
impl From<u32> for ABGR {
    #[allow(clippy::many_single_char_names)]
    fn from(i: u32) -> Self {
        let [a, b, g, r] = i.to_be_bytes();
        ABGR(a, b, g, r)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Timecode(pub Duration);
impl fmt::Display for Timecode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cs = self.0.subsec_millis() / 10;
        let s = self.0.as_secs() % 60;
        let m = (self.0.as_secs() / 60) % 60;
        let h = self.0.as_secs() / 3600;
        write!(f, "{:01}:{:02}:{:02}.{:02}", h, m % 60, s % 60, cs % 100)
    }
}
impl FromStr for Timecode {
    type Err = AssParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref TS_RE: Regex = Regex::new(r"(\d):(\d{2}):(\d{2})\.(\d{2})").unwrap();
        }
        let caps = TS_RE.captures(s).ok_or(BadTimeCode)?;
        let caps: Vec<u32> = caps
            .iter()
            .skip(1)
            .flatten()
            .map(|x| x.as_str().parse::<u32>().unwrap())
            .collect();

        let h = caps[0];
        let m = caps[1];
        let s = caps[2];
        let cs = caps[3];

        let result = (cs * 10) + (s * 1000) + (m * 60_000) + (h * 3_600_000);
        Ok(Timecode::from(result))
    }
}
impl From<u32> for Timecode {
    fn from(i: u32) -> Self {
        Timecode(Duration::from_millis(i.into()))
    }
}

pub fn split_line(s: &str) -> Result<(&str, &str), AssParseError> {
    lazy_static! {
        static ref S_RE: Regex = Regex::new(r"^(.+):\s+(.+)$").unwrap();
    }
    let caps = S_RE.captures(s).ok_or(BadLineFormat)?;
    Ok((caps.get(1).unwrap().as_str(), caps.get(2).unwrap().as_str()))
}
