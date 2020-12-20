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
        let yesno = |x| match x {
            true => "yes",
            false => "no",
        };
        macro_rules! field {
            ($field:ident, $name:literal) => {
                if let Some(x) = self.$field {
                    write!(f, concat!($name, ": {}"), x)?;
                }
            };
        }
        field!(title, "Title");
        field!(script_type, "ScriptType");
        field!(wrap_style, "WrapStyle");
        field!(play_res_x, "PlayResX");
        field!(play_res_y, "PlayResY");
        if let Some(x) = self.scaled_border_and_shadow {
            write!(f, "ScaledBorderAndShadow: {}", yesno(x))?;
        }
        field!(ycbcr_matrix, "YCbCr Matrix");
        field!(script, "Original Script");
        field!(translation, "Original Translation");
        field!(editing, "Original Editing");
        field!(timing, "Original Timing");
        field!(synch_point, "Synch Point");
        field!(updated_by, "Script Updated By");
        field!(update_details, "Update Details");
        field!(kerning, "Kerning");
        field!(language, "Language");
        Ok(())
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
