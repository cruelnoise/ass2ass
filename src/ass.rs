// ass.rs
// provides ass type definitions and associated parsing methods.
// exports the AssTrack type
// also some errors I guess? I don't know how to organize Rust projects.

use std::fmt;

use thiserror::Error;

//------------------------------------------------------------------------------
// errors
//------------------------------------------------------------------------------

// todo: make errors better

#[derive(Error, Debug, Clone, Copy, PartialEq)]
pub enum AssParseError {
    #[error("Invalid alignment value.")]
    BadAlignment,
    #[error("Invalid ass-style bool. -1 is true, 0 is false.")]
    BadAssBool,
    #[error("Invalid border style value.")]
    BadBorderStyle,
    #[error("Invalid colour code.")]
    BadColourCode,
    #[error("Invalid data for field type.")]
    BadConfigData,
    #[error("Unkown or unsupported config field.")]
    BadConfigField,
    #[error("Invalid encoding value.")]
    BadEncoding,
    #[error("Invalid event token in format line.")]
    BadEventToken,
    #[error("Line does not match ASS Field: Data format.")]
    BadLineFormat,
    #[error("Invalid style token in format line.")]
    BadStyleToken,
    #[error("Invalid time code.")]
    BadTimeCode,
    #[error("Invalid wrap style value.")]
    BadWrapStyle,
    #[error("Invalid or YCbCr Matrix value.")]
    BadYCbCrMatrix,
    #[error("Parser recieved illegal None state")]
    EnteredNoneState,
    #[error("Encountered a header while in an incompatible state.")]
    EncounteredIllegalHeader,
    #[error("Event line does not match format at token {0}")]
    EventNotMatchFormat(event::Token),
    #[error("Event line does not have entry for every field in Format.")]
    EventTooShort,
    #[error("Event has more fields than Format defines.")]
    EventTooLong,
    #[error("Attempted to parse line while in None state.")]
    NoParserState,
    #[error("Style line does not match format.")]
    StyleNotMatchFormat,
    #[error("The last token in an event format must be Text")]
    TextNotLastToken,
    #[error("Line is in unkown section header.")]
    UnknownSection,
}

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
        v.push(
            self.styleformat
                .as_ref()
                .unwrap_or(&style::Format::default())
                .to_string(),
        );
        for style in &self.styles {
            v.push(style.to_string());
        }
        v.push("".to_owned());
        v.push("[Events]".to_owned());
        v.push(
            self.eventformat
                .as_ref()
                .unwrap_or(&event::Format::default())
                .to_string(),
        );
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

mod common;
mod event;
mod info;
mod parser;
mod style;
