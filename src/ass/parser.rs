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
