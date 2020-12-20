use super::AssParseError;
use enum_default::EnumDefault;
use lazy_static::lazy_static;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use parse_display::Display;
use regex::Regex;
use std::{convert::TryInto, fmt, str::FromStr, time::Duration};

macro_rules! ass_num_enum {
    ($name:ident, $err:ident) => {
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", *self as u8)
            }
        }
        impl FromStr for $name {
            type Err = AssParseError;
            // okay yes this is a little less performant than the handwritten version but c'mon
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let n: u8 = s.parse().map_err(|_| AssParseError::$err)?;
                n.try_into().map_err(|_| AssParseError::$err)
            }
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, TryFromPrimitive, IntoPrimitive, EnumDefault)]
#[repr(u8)]
pub enum WrapStyle {
    #[default]
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
}

ass_num_enum!(WrapStyle, BadWrapStyle);

#[derive(Display, Debug, Clone, Copy, PartialEq)]
pub enum YCbCrMatrix {
    #[display("TV.601")]
    Bt601TV,
    #[display("PC.601")]
    Bt601PC,
    #[display("TV.709")]
    Bt709TV,
    #[display("PC.709")]
    Bt709PC,
    #[display("TV.240M")]
    Smpte240mTV,
    #[display("PC.240M")]
    Smpte240mPC,
    #[display("TV.FCC")]
    FccTV,
    #[display("PC.FCC")]
    FccPC,
}
impl FromStr for YCbCrMatrix {
    type Err = AssParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use YCbCrMatrix::*;
        Ok(match s.to_lowercase().as_str() {
            "tv.601" => Bt601TV,
            "pc.601" => Bt601PC,
            "tv.709" => Bt709TV,
            "pc.709" => Bt709PC,
            "tv.240m" => Smpte240mTV,
            "pc.240m" => Smpte240mPC,
            "tv.fcc" => FccTV,
            "pc.fcc" => FccPC,
            _ => return Err(AssParseError::BadYCbCrMatrix),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum BorderStyle {
    One = 1,
    Three = 3,
    Four = 4,
}

ass_num_enum!(BorderStyle, BadBorderStyle);

#[derive(Debug, Clone, Copy, PartialEq, TryFromPrimitive, IntoPrimitive, EnumDefault)]
#[repr(u8)]
pub enum Alignment {
    BottomLeft = 1,
    #[default]
    BottomCenter = 2,
    BottomRight = 3,
    Left = 4,
    Center = 5,
    Right = 6,
    TopLeft = 7,
    TopCenter = 8,
    TopRight = 9,
}

ass_num_enum!(Alignment, BadAlignment);

#[derive(Debug, Clone, Copy, PartialEq, TryFromPrimitive, IntoPrimitive, EnumDefault)]
#[repr(u8)]
pub enum Encoding {
    Ansi = 0,
    #[default]
    Default = 1,
    Symbol = 2,
    Mac = 77,
    ShiftJIS = 128,
    Hangeul = 129,
    Johab = 130,
    Gb2312 = 134,
    ChineseBIG5 = 136,
    Greek = 161,
    Turkish = 162,
    Vietnamese = 163,
    Hebrew = 177,
    Arabic = 178,
    Baltic = 186,
    Russian = 204,
    Thai = 222,
    EastEuropean = 238,
    Oem = 255,
}

ass_num_enum!(Encoding, BadEncoding);

#[derive(Display, Debug, Clone, Copy, PartialEq, Default)]
#[display("&H{0:02X}{1:02X}{2:02X}{3:02X}")]
pub struct ABGR(u8, u8, u8, u8);
impl FromStr for ABGR {
    type Err = AssParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: both renderers are more permissive than this
        if s.len() == 10 && &s[..2] == "&H" {
            if let Ok(i) = u32::from_str_radix(&s[2..], 16) {
                return Ok(Self::from(i));
            }
        }

        Err(AssParseError::BadColourCode)
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
        let caps = TS_RE.captures(s).ok_or(AssParseError::BadTimeCode)?;
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
    let caps = S_RE.captures(s).ok_or(AssParseError::BadLineFormat)?;
    Ok((caps.get(1).unwrap().as_str(), caps.get(2).unwrap().as_str()))
}
