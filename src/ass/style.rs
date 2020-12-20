use self::Token::*;
use super::common::{Alignment, BorderStyle, Encoding, ABGR};
use super::AssParseError::{self, BadAssBool, BadStyleToken, StyleNotMatchFormat};
use parse_display::Display;
use std::{fmt, str::FromStr};

#[derive(Display, Debug, Clone, Copy, PartialEq)]
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
