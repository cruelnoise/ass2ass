use std::error::Error;
use std::fs;

mod ass;
use ass::AssTrack;

pub struct Config {
    pub input: String,
    pub output: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, String> {
        if args.len() != 3 {
            return Err("not enough arguments".to_owned());
        }

        let input = &args[1];
        if !input.ends_with(".ass") {
            return Err(format!("file extension must be .ass : {}", input));
        }

        let output = &args[2];
        if !output.ends_with(".ass") {
            return Err(format!("file extension must be .ass : {}", output));
        }

        let (input, output) = (input.into(), output.into());
        Ok(Self { input, output })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let instring = fs::read_to_string(config.input)?;
    let instring = strip_bom(instring.as_str());

    let track = AssTrack::parse_track(&instring)?;

    let outstring = track.to_string();

    fs::write(config.output, outstring)?;

    Ok(())
}

fn strip_bom(s: &str) -> &str {
    s.strip_prefix("\u{feff}").unwrap_or(s)
}
