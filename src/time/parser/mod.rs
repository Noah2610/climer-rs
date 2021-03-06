use regex::Regex;

use super::{Time, TimeBuilder};
use crate::error::{ClimerError, ClimerResult};
use crate::settings::parser::codes::*;

pub fn parse_time<T, U>(time: T, format_opt: Option<U>) -> ClimerResult<Time>
where
    T: ToString,
    U: ToString,
{
    Ok(if let Some(format) = format_opt {
        parse_time_with_format(time, format)?
    } else {
        parse_time_without_format(time)?
    })
}

fn parse_time_without_format<T>(time: T) -> ClimerResult<Time>
where
    T: ToString,
{
    let time = time.to_string();
    let replace_re = Regex::new(r"\s").unwrap();
    let time = &replace_re.replace_all(&time, "");
    let mut builder = TimeBuilder::new();
    let mut remaining_input = time.to_string();
    let input_re = Regex::new(r"(?P<num>\d+)(?P<ident>[a-zA-Z]+)").unwrap();
    for caps in input_re.captures_iter(time) {
        remaining_input = remaining_input.replace(&caps[0], "");
        let num = caps["num"].parse().expect("Should unwrap to integer");
        match &caps["ident"] {
            HOUR => builder = builder.hours(num),
            MINUTE => builder = builder.minutes(num),
            SECOND => builder = builder.seconds(num),
            MILLISECOND => builder = builder.milliseconds(num),
            NANOSECOND => builder = builder.nanoseconds(num),
            _ => {
                return Err(ClimerError::InvalidTimeIdentifier(
                    caps["ident"].to_string(),
                ))
            }
        }
    }

    if !remaining_input.is_empty() {
        return Err(ClimerError::InvalidInput(remaining_input));
    }

    Ok(builder.build())
}

fn parse_time_with_format<T, U>(_time: T, _format: U) -> ClimerResult<Time>
where
    T: ToString,
    U: ToString,
{
    Err(ClimerError::Unimplemented("--format".to_string()))
}

#[cfg(test)]
mod tests;
