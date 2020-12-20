// ass.rs
// provides ass type definitions and associated parsing methods.
// exports the AssTrack type
// also some errors I guess? I don't know how to organize Rust projects.

use std::{error::Error, fmt};

//------------------------------------------------------------------------------
// errors
//------------------------------------------------------------------------------

// todo: make errors better

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AssParseError {
    BadAlignment,
    BadAssBool,
    BadBorderStyle,
    BadColourCode,
    BadConfigData,
    BadConfigField,
    BadEncoding,
    BadEventToken,
    BadLineFormat,
    BadStyleToken,
    BadTimeCode,
    BadWrapStyle,
    BadYCbCrMatrix,
    EnteredNoneState,
    EncounteredIllegalHeader,
    EventNotMatchFormat(event::Token),
    EventTooShort,
    EventTooLong,
    NoParserState,
    StyleNotMatchFormat,
    TextNotLastToken,
    UnknownSection,
}
impl fmt::Display for AssParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use AssParseError::*;
        let s = match *self {
            BadAlignment => "Invalid alignment value.".to_owned(),
            BadAssBool => "Invalid ass-style bool. -1 is true, 0 is false.".to_owned(),
            BadBorderStyle => "Invalid border style value.".to_owned(),
            BadColourCode => "Invalid colour code.".to_owned(),
            BadConfigData => "Invalid data for field type.".to_owned(),
            BadConfigField => "Unkown or unsupported config field.".to_owned(),
            BadEncoding => "Invalid encoding value.".to_owned(),
            BadEventToken => "Invalid event token in format line.".to_owned(),
            BadLineFormat => "Line does not match ASS Field: Data format.".to_owned(),
            BadStyleToken => "Invalid style token in format line.".to_owned(),
            BadTimeCode => "Invalid time code.".to_owned(),
            BadWrapStyle => "Invalid wrap style value.".to_owned(),
            BadYCbCrMatrix => "Invalid or YCbCr Matrix value.".to_owned(),
            EnteredNoneState => "Parser recieved illegal None state".to_owned(),
            EncounteredIllegalHeader => "Encountered a header while in an incompatible state.".to_owned(),
            EventNotMatchFormat(t) => format!("Event line does not match format at token {}", t),
            EventTooShort => "Event line does not have entry for every field in Format.".to_owned(),
            EventTooLong => "Event has more fields than Format defines.".to_owned(),
            NoParserState => "Attempted to parse line while in None state.".to_owned(),
            StyleNotMatchFormat => "Style line does not match format.".to_owned(),
            TextNotLastToken => "The last token in an event format must be Text".to_owned(),
            UnknownSection => "Line is in unkown section header.".to_owned(),
        };
        write!(
            f,
            "{}",
            s
        )
    }
}
impl Error for AssParseError {}

#[derive(Debug, Clone, Default)]
pub struct AssTrack<'a> {
    header: info::Header<'a>,
    styleformat: Option<style::Format>,
    styles: Vec<style::Style<'a>>,
    eventformat: Option<event::Format>,
    events: Vec<event::Event<'a>>,
}
impl fmt::Display for AssTrack<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut v = Vec::<String>::new();
        v.push("[Script Info]".to_owned());
        v.push(self.header.to_string());
        v.push("".to_owned());
        v.push("[V4+ Styles]".to_owned());
        v.push(self.styleformat.as_ref().unwrap_or(&style::Format::default()).to_string());
        for style in &self.styles {
            v.push(style.to_string());
        }
        v.push("".to_owned());
        v.push("[Events]".to_owned());
        v.push(self.eventformat.as_ref().unwrap_or(&event::Format::default()).to_string());
        for event in &self.events {
            v.push(event.to_string());
        }
        write!(f, "{}", v.join("\n"))
    }
}
impl<'a> AssTrack<'a> {
    pub fn parse_track(s: &'a str) -> Result<AssTrack<'a>, AssParseError> {
        parser::parse_track(s)
    }
}

mod common {
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

    pub fn split_line<'a>(s: &'a str) -> Result<(&'a str, &'a str), AssParseError> {
        lazy_static! {
            static ref S_RE: Regex = Regex::new(r"^(.+):\s+(.+)$").unwrap();
        }
        let caps = S_RE.captures(s).ok_or(BadLineFormat)?;
        Ok((caps.get(1).unwrap().as_str(), caps.get(2).unwrap().as_str()))
    }
}

mod parser {
    use super::common::split_line;
    use super::AssParseError::{
        self, EncounteredIllegalHeader, EnteredNoneState, NoParserState, UnknownSection,
    };
    use super::AssTrack;
    use super::{event, info, style};
    use lazy_static::lazy_static;
    use regex::Regex;

    #[derive(Debug, Clone, PartialEq)]
    pub enum ParserState {
        None,
        Info,
        Styles,
        Events,
        Other(String),
    }

    struct AssParser<'a> {
        state: ParserState,
        track: AssTrack<'a>,
        previous_states: Vec<ParserState>,
    }
    impl AssParser<'_> {
        fn switch_state(&mut self, new_state: ParserState) -> Result<(), AssParseError> {
            use ParserState::*;
            match new_state {
                None => Err(EnteredNoneState),
                Info => {
                    if self.state != None {
                        Err(EncounteredIllegalHeader)
                    } else {
                        self.previous_states.push(self.state.clone());
                        self.state = Info;
                        Ok(())
                    }
                }
                Styles => {
                    if self.previous_states.iter().any(|x| *x == Events) {
                        Err(EncounteredIllegalHeader)
                    } else {
                        self.previous_states.push(self.state.clone());
                        self.state = Styles;
                        Ok(())
                    }
                }
                Events => {
                    self.previous_states.push(self.state.clone());
                    self.state = Events;
                    Ok(())
                }
                Other(s) => {
                    eprintln!("Entered unknown header {}", s);
                    self.state = Other(s);
                    Ok(())
                }
            }
        }
    }

    pub fn parse_track<'a>(s: &'a str) -> Result<AssTrack<'a>, AssParseError> {
        let mut parser = AssParser {
            state: ParserState::None,
            track: AssTrack::<'a>::default(),
            previous_states: Vec::<ParserState>::new(),
        };

        // was too tired to write this. split into another function.
        for (line_n, line) in s.lines().enumerate() {
            let line = line.trim();
            if line == "" {
                continue;
            }
            if let Err(e) = parse_line(&mut parser, line) {
                match e {
                    NoParserState | EncounteredIllegalHeader => return Err(e),
                    _ => eprintln!("Dropped line {}: {}", line_n, e),
                };
            }
        }
        Ok(parser.track)
    }

    fn parse_line<'a>(parser: &mut AssParser<'a>, line: &'a str) -> Result<(), AssParseError> {
        use ParserState::*;
        lazy_static! {
            static ref H_RE: Regex = Regex::new(r"^\[.+\]$").unwrap();
        }
        let line = line.trim();
        if H_RE.is_match(line) {
            match line {
                "[Script Info]" => parser.switch_state(Info),
                "[V4+ Styles]" => parser.switch_state(Styles),
                "[Events]" => parser.switch_state(Events),
                _ => parser.switch_state(Other(line.to_owned())),
            }
        } else {
            let (field, data) = split_line(line)?;
            match parser.state {
                None => return Err(NoParserState),
                Info => parser
                    .track
                    .header
                    .set(info::ConfigKind::parse(field, data)?),
                Styles => {
                    if field == "Format" {
                        parser.track.styleformat = Some(data.parse()?)
                    } else {
                        if parser.track.styleformat.is_none() {
                            parser.track.styleformat = Some(style::Format::default())
                        }
                        parser.track.styles.push(style::Style::parse(
                            data,
                            parser.track.styleformat.as_ref(),
                        )?)
                    }
                }
                Events => {
                    if field == "Format" {
                        parser.track.eventformat = Some(data.parse()?)
                    } else {
                        if parser.track.eventformat.is_none() {
                            parser.track.eventformat = Some(event::Format::default())
                        }
                        parser.track.events.push(event::Event::parse(
                            data,
                            Some(field),
                            parser.track.eventformat.as_ref(),
                        )?)
                    }
                }
                Other(_) => return Err(UnknownSection),
            };
            Ok(())
        }
    }
}

mod info {
    use self::ConfigKind::*;
    use super::common::{WrapStyle, YCbCrMatrix};
    use super::AssParseError::{self, BadConfigData, BadConfigField};
    use std::fmt;

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum ConfigKind<'a> {
        Title(&'a str),
        ScriptType(&'a str),
        WrapStyle(WrapStyle),
        PlayResX(u32),
        PlayResY(u32),
        ScaledBorderAndShadow(bool),
        YCbCrMatrix(YCbCrMatrix),
        ScriptCredit(&'a str),
        TranslationCredit(&'a str),
        EditingCredit(&'a str),
        TimingCredit(&'a str),
        SynchPoint(&'a str),
        UpdateCredit(&'a str),
        UpdateDetails(&'a str),
        Kerning(bool),
        Language(&'a str), //can't be assed to enumerate
    }
    impl<'a> ConfigKind<'a> {
        pub fn parse(field: &'a str, data: &'a str) -> Result<Self, AssParseError> {
            Ok(match field {
                "Title" => Title(data),
                "ScriptType" => ScriptType(data),
                "WrapStyle" => WrapStyle(data.parse()?),
                "PlayResX" => PlayResX(data.parse::<u32>().or(Err(BadConfigData))?),
                "PlayResY" => PlayResY(data.parse::<u32>().or(Err(BadConfigData))?),
                "ScaledBorderAndShadow" => ScaledBorderAndShadow(match data {
                    "yes" => true,
                    "no" => false,
                    _ => return Err(BadConfigData),
                }),
                "YCbCr Matrix" => YCbCrMatrix(data.parse()?),
                "Original Script" => ScriptCredit(data),
                "Original Translation" => TranslationCredit(data),
                "Original Editing" => EditingCredit(data),
                "Original Timing" => TimingCredit(data),
                "Synch Point" => SynchPoint(data),
                "Script Updated By" => UpdateCredit(data),
                "Update Details" => UpdateDetails(data),
                "Kerning" => Kerning(match data {
                    "yes" => true,
                    "no" => false,
                    _ => return Err(BadConfigData),
                }),
                "Language" => Language(data),
                _ => return Err(BadConfigField),
            })
        }
    }
    #[derive(Debug, Clone, Copy, Default)]
    pub struct Header<'a> {
        title: Option<&'a str>,
        script_type: Option<&'a str>,
        wrap_style: Option<WrapStyle>,
        play_res_x: Option<u32>,
        play_res_y: Option<u32>,
        scaled_border_and_shadow: Option<bool>,
        ycbcr_matrix: Option<YCbCrMatrix>,
        script: Option<&'a str>,
        translation: Option<&'a str>,
        editing: Option<&'a str>,
        timing: Option<&'a str>,
        synch_point: Option<&'a str>,
        updated_by: Option<&'a str>,
        update_details: Option<&'a str>,
        kerning: Option<bool>,
        language: Option<&'a str>,
    }
    impl fmt::Display for Header<'_> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut v = Vec::<String>::new();
            let yesno = |x| match x {
                true => "yes",
                false => "no",
            };
            if let Some(x) = self.title {
                v.push(format!("Title: {}", x))
            }
            if let Some(x) = self.script_type {
                v.push(format!("ScriptType: {}", x))
            }
            if let Some(x) = self.wrap_style {
                v.push(format!("WrapStyle: {}", x))
            }
            if let Some(x) = self.play_res_x {
                v.push(format!("PlayResX: {}", x))
            }
            if let Some(x) = self.play_res_y {
                v.push(format!("PlayResY: {}", x))
            }
            if let Some(x) = self.scaled_border_and_shadow {
                v.push(format!("ScaledBorderAndShadow: {}", yesno(x)))
            }
            if let Some(x) = self.ycbcr_matrix {
                v.push(format!("YCbCr Matrix: {}", x))
            }
            if let Some(x) = self.script {
                v.push(format!("Original Script: {}", x))
            }
            if let Some(x) = self.translation {
                v.push(format!("Original Translation: {}", x))
            }
            if let Some(x) = self.editing {
                v.push(format!("Original Editing: {}", x))
            }
            if let Some(x) = self.timing {
                v.push(format!("Original Timing: {}", x))
            }
            if let Some(x) = self.synch_point {
                v.push(format!("Synch Point: {}", x))
            }
            if let Some(x) = self.updated_by {
                v.push(format!("Script Updated By: {}", x))
            }
            if let Some(x) = self.update_details {
                v.push(format!("Update Details: {}", x))
            }
            if let Some(x) = self.kerning {
                v.push(format!("Kerning: {}", x))
            }
            if let Some(x) = self.language {
                v.push(format!("Language: {}", x))
            }
            write! {
                f,"{}",v.join("\n")

            }
        }
    }
    impl<'a> Header<'a> {
        pub fn set(&mut self, c: ConfigKind<'a>) {
            match c {
                Title(x) => {
                    self.title.get_or_insert(x);
                }
                ScriptType(x) => {
                    self.script_type.get_or_insert(x);
                }
                WrapStyle(x) => {
                    self.wrap_style.get_or_insert(x);
                }
                PlayResX(x) => {
                    self.play_res_x.get_or_insert(x);
                }
                PlayResY(x) => {
                    self.play_res_y.get_or_insert(x);
                }
                ScaledBorderAndShadow(x) => {
                    self.scaled_border_and_shadow.get_or_insert(x);
                }
                YCbCrMatrix(x) => {
                    self.ycbcr_matrix.get_or_insert(x);
                }
                ScriptCredit(x) => {
                    self.script.get_or_insert(x);
                }
                TranslationCredit(x) => {
                    self.translation.get_or_insert(x);
                }
                EditingCredit(x) => {
                    self.editing.get_or_insert(x);
                }
                TimingCredit(x) => {
                    self.timing.get_or_insert(x);
                }
                SynchPoint(x) => {
                    self.synch_point.get_or_insert(x);
                }
                UpdateCredit(x) => {
                    self.updated_by.get_or_insert(x);
                }
                UpdateDetails(x) => {
                    self.update_details.get_or_insert(x);
                }
                Kerning(x) => {
                    self.kerning.get_or_insert(x);
                }
                Language(x) => {
                    self.language.get_or_insert(x);
                }
            };
        }
    }
}

mod style {
    use self::Token::*;
    use super::common::{Alignment, BorderStyle, Encoding, ABGR};
    use super::AssParseError::{self, BadAssBool, BadStyleToken, StyleNotMatchFormat};
    use std::{fmt, str::FromStr};

    #[derive(Debug, Clone, Copy, PartialEq)]
    enum Token {
        Name,
        Fontname,
        Fontsize,
        PrimaryColour,
        SecondaryColour,
        OutlineColour,
        BackColour,
        Bold,
        Italic,
        Underline,
        StrikeOut,
        ScaleX,
        ScaleY,
        Spacing,
        Angle,
        BorderStyle,
        Outline,
        Shadow,
        Alignment,
        MarginL,
        MarginR,
        MarginV,
        Encoding,
    }
    impl fmt::Display for Token {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "{}",
                match *self {
                    Name => "Name",
                    Fontname => "Fontname",
                    Fontsize => "Fontsize",
                    PrimaryColour => "PrimaryColour",
                    SecondaryColour => "SecondaryColour",
                    OutlineColour => "OutlineColour",
                    BackColour => "BackColour",
                    Bold => "Bold",
                    Italic => "Italic",
                    Underline => "Underline",
                    StrikeOut => "StrikeOut",
                    ScaleX => "ScaleX",
                    ScaleY => "ScaleY",
                    Spacing => "Spacing",
                    Angle => "Angle",
                    BorderStyle => "BorderStyle",
                    Outline => "Outline",
                    Shadow => "Shadow",
                    Alignment => "Alignment",
                    MarginL => "MarginL",
                    MarginR => "MarginR",
                    MarginV => "MarginV",
                    Encoding => "Encoding",
                }
            )
        }
    }
    impl FromStr for Token {
        type Err = AssParseError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(match s {
                "Name" => Name,
                "Fontname" => Fontname,
                "Fontsize" => Fontsize,
                "PrimaryColour" => PrimaryColour,
                "SecondaryColour" => SecondaryColour,
                "OutlineColour" => OutlineColour,
                "BackColour" => BackColour,
                "Bold" => Bold,
                "Italic" => Italic,
                "Underline" => Underline,
                "StrikeOut" => StrikeOut,
                "ScaleX" => ScaleX,
                "ScaleY" => ScaleY,
                "Spacing" => Spacing,
                "Angle" => Angle,
                "BorderStyle" => BorderStyle,
                "Outline" => Outline,
                "Shadow" => Shadow,
                "Alignment" => Alignment,
                "MarginL" => MarginL,
                "MarginR" => MarginR,
                "MarginV" => MarginV,
                "Encoding" => Encoding,
                _ => return Err(BadStyleToken),
            })
        }
    }

    #[derive(Debug, Clone)]
    pub struct Format(Vec<Token>);
    impl fmt::Display for Format {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "Format: {}",
                self.0
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            )
        }
    }
    impl FromStr for Format {
        type Err = AssParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut f = Format::new();
            for t in s.split(',') {
                f.push(t.trim().parse()?);
            }
            Ok(f)
        }
    }
    impl Default for Format {
        fn default() -> Self {
            Format(vec![
                Name,
                Fontname,
                Fontsize,
                PrimaryColour,
                SecondaryColour,
                OutlineColour,
                BackColour,
                Bold,
                Italic,
                Underline,
                StrikeOut,
                ScaleX,
                ScaleY,
                Spacing,
                Angle,
                BorderStyle,
                Outline,
                Shadow,
                Alignment,
                MarginL,
                MarginR,
                MarginV,
                Encoding,
            ])
        }
    }
    impl Format {
        fn new() -> Format {
            Format(Vec::<Token>::new())
        }
        fn push(&mut self, t: Token) {
            self.0.push(t)
        }
    }

    #[derive(Debug, Clone)]
    pub struct Style<'a> {
        format: Format,
        name: &'a str,
        font_name: &'a str,
        font_size: u32,
        primary_colour: ABGR,
        secondary_colour: ABGR,
        outline_colour: ABGR,
        back_colour: ABGR,
        bold: bool,
        italic: bool,
        underline: bool,
        strikeout: bool,
        scale_x: u32,
        scale_y: u32,
        spacing: f64,
        angle: u32,
        border_style: BorderStyle,
        outline: f64,
        shadow: f64,
        alignment: Alignment,
        margin_l: i32,
        margin_r: i32,
        margin_v: i32,
        encoding: Encoding,
    }
    impl fmt::Display for Style<'_> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Style: {}", {
                self.format
                    .0
                    .iter()
                    .map(|x| match x {
                        Name => self.name.to_owned(),
                        Fontname => self.font_name.to_owned(),
                        Fontsize => self.font_size.to_string(),
                        PrimaryColour => self.primary_colour.to_string(),
                        SecondaryColour => self.secondary_colour.to_string(),
                        OutlineColour => self.outline_colour.to_string(),
                        BackColour => self.back_colour.to_string(),
                        Bold => bool_to_ass_bool(self.bold),
                        Italic => bool_to_ass_bool(self.italic),
                        Underline => bool_to_ass_bool(self.underline),
                        StrikeOut => bool_to_ass_bool(self.strikeout),
                        ScaleX => self.scale_x.to_string(),
                        ScaleY => self.scale_y.to_string(),
                        Spacing => self.spacing.to_string(),
                        Angle => self.angle.to_string(),
                        BorderStyle => self.border_style.to_string(),
                        Outline => self.outline.to_string(),
                        Shadow => self.shadow.to_string(),
                        Alignment => self.alignment.to_string(),
                        MarginL => self.margin_l.to_string(),
                        MarginR => self.margin_r.to_string(),
                        MarginV => self.margin_v.to_string(),
                        Encoding => self.encoding.to_string(),
                    })
                    .collect::<Vec<String>>()
                    .join(",")
            })
        }
    }
    impl Default for Style<'_> {
        fn default() -> Self {
            Style {
                format: Format::default(),
                name: "Default",
                font_name: "Arial",
                font_size: 18,
                primary_colour: ABGR::from(0xffffff00),
                secondary_colour: ABGR::from(0x00ffff00),
                outline_colour: ABGR::from(0x00000000),
                back_colour: ABGR::from(0x00000000),
                bold: false,
                italic: false,
                underline: false,
                strikeout: false,
                scale_x: 100,
                scale_y: 100,
                spacing: 0.0,
                angle: 0,
                border_style: BorderStyle::One,
                outline: 2.0,
                shadow: 3.0,
                alignment: Alignment::BottomCenter,
                margin_l: 20,
                margin_r: 20,
                margin_v: 20,
                encoding: Encoding::Default,
            }
        }
    }
    impl<'a> Style<'a> {
        pub fn parse(s: &'a str, f: Option<&Format>) -> Result<Style<'a>, AssParseError> {
            let mut res = Style::default();
            if let Some(v) = f {
                res.format = v.clone();
            }
            let data: Vec<&str> = s.split(',').collect();
            if data.len() != res.format.0.len() {
                return Err(StyleNotMatchFormat);
            }
            for (token, value) in res.format.0.iter().zip(data.iter()) {
                match token {
                    Name => res.name = value,
                    Fontname => res.font_name = value,
                    Fontsize => res.font_size = value.parse().or(Err(StyleNotMatchFormat))?,
                    PrimaryColour => res.primary_colour = value.parse()?,
                    SecondaryColour => res.secondary_colour = value.parse()?,
                    OutlineColour => res.outline_colour = value.parse()?,
                    BackColour => res.back_colour = value.parse()?,
                    Bold => res.bold = ass_bool_to_bool(value)?,
                    Italic => res.italic = ass_bool_to_bool(value)?,
                    Underline => res.underline = ass_bool_to_bool(value)?,
                    StrikeOut => res.strikeout = ass_bool_to_bool(value)?,
                    ScaleX => res.scale_x = value.parse().or(Err(StyleNotMatchFormat))?,
                    ScaleY => res.scale_y = value.parse().or(Err(StyleNotMatchFormat))?,
                    Spacing => res.spacing = value.parse().or(Err(StyleNotMatchFormat))?,
                    Angle => res.angle = value.parse().or(Err(StyleNotMatchFormat))?,
                    BorderStyle => res.border_style = value.parse()?,
                    Outline => res.outline = value.parse().or(Err(StyleNotMatchFormat))?,
                    Shadow => res.shadow = value.parse().or(Err(StyleNotMatchFormat))?,
                    Alignment => res.alignment = value.parse()?,
                    MarginL => res.margin_l = value.parse().or(Err(StyleNotMatchFormat))?,
                    MarginR => res.margin_r = value.parse().or(Err(StyleNotMatchFormat))?,
                    MarginV => res.margin_v = value.parse().or(Err(StyleNotMatchFormat))?,
                    Encoding => res.encoding = value.parse()?,
                };
            }
            Ok(res)
        }
    }

    fn ass_bool_to_bool(s: &str) -> Result<bool, AssParseError> {
        Ok(match s {
            "-1" => true,
            "0" => false,
            _ => return Err(BadAssBool),
        })
    }

    fn bool_to_ass_bool(b: bool) -> String {
        match b {
            true => "-1",
            false => "0",
        }
        .to_owned()
    }
}

mod event {
    use self::Token::*;
    use super::common::Timecode;
    use super::AssParseError::{self, EventTooLong, EventTooShort, BadEventToken, EventNotMatchFormat, TextNotLastToken};
    use std::{fmt, str::FromStr};

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Token {
        Layer,
        Start,
        End,
        Style,
        Name,
        MarginL,
        MarginR,
        MarginV,
        Effect,
        Text,
    }
    impl fmt::Display for Token {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "{}",
                match *self {
                    Layer => "Layer",
                    Start => "Start",
                    End => "End",
                    Style => "Style",
                    Name => "Name",
                    MarginL => "MarginL",
                    MarginR => "MarginR",
                    MarginV => "MarginV",
                    Effect => "Effect",
                    Text => "Text",
                }
            )
        }
    }
    impl FromStr for Token {
        type Err = AssParseError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(match s {
                "Layer" => Layer,
                "Start" => Start,
                "End" => End,
                "Style" => Style,
                "Name" => Name,
                "MarginL" => MarginL,
                "MarginR" => MarginR,
                "MarginV" => MarginV,
                "Effect" => Effect,
                "Text" => Text,
                _ => return Err(BadEventToken),
            })
        }
    }

    #[derive(Debug, Clone)]
    pub struct Format(Vec<Token>);
    impl fmt::Display for Format {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "Format: {}",
                self.0
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            )
        }
    }
    // Should make this generic across style and event.
    impl FromStr for Format {
        type Err = AssParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut f = Format::new();
            for t in s.split(',') {
                f.push(t.trim().parse()?);
            }
            if f.0.len() < 1 || *f.0.last().unwrap() != Text {
                return Err(TextNotLastToken);
            }
            Ok(f)
        }
    }
    impl Default for Format {
        fn default() -> Self {
            Format(vec![
                Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text,
            ])
        }
    }
    impl Format {
        fn new() -> Format {
            Format(Vec::<Token>::new())
        }
        fn push(&mut self, t: Token) {
            self.0.push(t)
        }
    }

    #[derive(Debug, Clone)]
    pub struct Event<'a> {
        format: Format,
        start_time: Timecode,
        end_time: Timecode,

        descriptor: &'a str,
        layer: u32,
        style: Option<&'a str>,
        actor: Option<&'a str>,
        margin_l: i32,
        margin_r: i32,
        margin_v: i32,
        effect: Option<&'a str>,
        text: Option<&'a str>,
    }
    impl fmt::Display for Event<'_> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}: {}", self.descriptor, {
                self.format
                    .0
                    .iter()
                    .map(|x| match x {
                        Layer => self.layer.to_string(),
                        Start => self.start_time.to_string(),
                        End => self.end_time.to_string(),
                        Style => self.style.unwrap_or("").to_owned(),
                        Name => self.actor.unwrap_or("").to_owned(),
                        MarginL => self.margin_l.to_string(),
                        MarginR => self.margin_r.to_string(),
                        MarginV => self.margin_v.to_string(),
                        Effect => self.effect.unwrap_or("").to_owned(),
                        Text => self.text.unwrap_or("").to_owned(),
                    })
                    .collect::<Vec<String>>()
                    .join(",")
            })
        }
    }
    impl Default for Event<'_> {
        fn default() -> Self {
            Event {
                format: Format::default(),
                start_time: Timecode::from(0),
                end_time: Timecode::from(10_000),
                descriptor: "Dialogue",
                layer: 0,
                style: Some("Default"),
                actor: None,
                margin_l: 0,
                margin_r: 0,
                margin_v: 0,
                effect: None,
                text: None,
            }
        }
    }
    impl<'a> Event<'a> {
        pub fn parse(
            s: &'a str,
            d: Option<&'a str>,
            f: Option<&Format>,
        ) -> Result<Event<'a>, AssParseError> {
            let mut res = Event::default();
            if let Some(v) = d {
                res.descriptor = v;
            }
            if let Some(v) = f {
                res.format = v.clone();
            }
            let mut data = Vec::<&str>::new();
            let iter = s.split(",");
            for (n, v) in iter.enumerate() {
                if n < res.format.0.len() - 1 {
                    data.push(v)
                }
                else {
                    break;
                }
            }
            data.push(&s[data.join(",").len() + 1..]);
            if data.len() < res.format.0.len() {
                return Err(EventTooShort);
            }
            else if data.len() > res.format.0.len() {
                eprintln!("{:?}", data);
                return Err(EventTooLong);
            }
            for (token, value) in res.format.0.iter().zip(data.iter()) {
                match token {
                    Layer => res.layer = value.parse().or(Err(EventNotMatchFormat(Layer)))?,
                    Start => res.start_time = value.parse().or(Err(EventNotMatchFormat(Start)))?,
                    End => res.end_time = value.parse().or(Err(EventNotMatchFormat(End)))?,
                    Style => res.style = Some(value),
                    Name => res.actor = Some(value),
                    MarginL => res.margin_l = value.parse().or(Err(EventNotMatchFormat(MarginL)))?,
                    MarginR => res.margin_r = value.parse().or(Err(EventNotMatchFormat(MarginR)))?,
                    MarginV => res.margin_v = value.parse().or(Err(EventNotMatchFormat(MarginV)))?,
                    Effect => res.effect = Some(value),
                    Text => res.text = Some(value),
                };
            }
            Ok(res)
        }
    }
}
