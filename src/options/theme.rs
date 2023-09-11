use crate::options::{vars, Vars, OptionsError};
use crate::theme::{Options, UseColours, ColourScale, Definitions};

use super::parser::Opts;


impl Options {
    pub fn deduce<V: Vars>(matches: &Opts, vars: &V) -> Result<Self, OptionsError> {
        let use_colours = UseColours::deduce(matches, vars)?;
        let colour_scale = ColourScale::deduce(matches);

        let definitions = if use_colours == UseColours::Never {
                Definitions::default()
            }
            else {
                Definitions::deduce(vars)
            };

        Ok(Self { use_colours, colour_scale, definitions })
    }
}


impl UseColours {
    fn deduce<V: Vars>(matches: &Opts, vars: &V) -> Result<Self, OptionsError> {
        let default_value = match vars.get(vars::NO_COLOR) {
            Some(_) => Self::Never,
            None => Self::Automatic,
        };

        let Some(ref word) = matches.color else { return Ok(default_value) };

        if word == "always" {
            Ok(Self::Always)
        }
        else if word == "auto" || word == "automatic" {
            Ok(Self::Automatic)
        }
        else if word == "never" {
            Ok(Self::Never)
        }
        else {
            Err(OptionsError::BadArgument("--color".to_string(), word.to_string_lossy().to_string()))
        }
    }
}


impl ColourScale {
    fn deduce(matches: &Opts) -> Self {
        if matches.color_scale > 0 {
            return Self::Gradient;
        }
        Self::Fixed
    }
}


impl Definitions {
    fn deduce<V: Vars>(vars: &V) -> Self {
        let ls =  vars.get(vars::LS_COLORS) .map(|e| e.to_string_lossy().to_string());
        let exa = vars.get(vars::EXA_COLORS).map(|e| e.to_string_lossy().to_string());
        Self { ls, exa }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deduce_colour_scale() {
        let matches = Opts {
            ..Opts::default()
        };

        assert_eq!(ColourScale::deduce(&matches), ColourScale::Fixed);
    }

    #[test]
    fn deduce_colour_scale_on() {
        let matches = Opts {
            color_scale: 1,
            ..Opts::default()
        };

        assert_eq!(ColourScale::deduce(&matches), ColourScale::Gradient);
    }
}
