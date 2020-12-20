use self::Token::*;
use super::common::Timecode;
use super::AssParseError::{
    self, BadEventToken, EventNotMatchFormat, EventTooLong, EventTooShort, TextNotLastToken,
};
use parse_display::Display;
use smart_default::SmartDefault;
use std::{cmp::Ordering, fmt, str::FromStr};

#[derive(Display, Debug, Clone, Copy, PartialEq)]
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
        if f.0.is_empty() || *f.0.last().unwrap() != Text {
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

#[derive(Debug, Clone, SmartDefault)]
pub struct Event<'a> {
    format: Format,
    start_time: Timecode,
    #[default(Timecode::from(10_000))]
    end_time: Timecode,

    #[default("Dialogue")]
    descriptor: &'a str,
    layer: u32,
    #[default(Some("Default"))]
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
        let iter = s.split(',');
        for (n, v) in iter.enumerate() {
            if n < res.format.0.len() - 1 {
                data.push(v)
            } else {
                break;
            }
        }
        data.push(&s[data.join(",").len() + 1..]);
        match data.len().cmp(&res.format.0.len()) {
            Ordering::Greater => return Err(EventTooLong),
            Ordering::Less => return Err(EventTooShort),
            Ordering::Equal => (),
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
