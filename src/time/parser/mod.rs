use regex::Regex;

use crate::error::{ ClimerResult, ClimerError };
use crate::settings::parser::codes::*;
use super::{ Time, TimeBuilder };

pub fn parse_time(time: &str, format_opt: Option<&str>) -> ClimerResult<Time> {
    Ok(if let Some(format) = format_opt {
        parse_time_with_format(time, format)?
    } else {
        parse_time_without_format(time)?
    })
}

fn parse_time_without_format(time: &str) -> ClimerResult<Time> {
    let mut builder = TimeBuilder::new();
    let re = Regex::new(r"(?P<num>\d+)(?P<ident>[a-zA-Z]+)").unwrap();
    for caps in re.captures_iter(time) {
        let num = caps["num"].parse().expect("Should unwrap to integer");
        match &caps["ident"] {
            HOUR        => builder = builder.hours(num),
            MINUTE      => builder = builder.minutes(num),
            SECOND      => builder = builder.seconds(num),
            MILLISECOND => builder = builder.milliseconds(num),
            NANOSECOND  => builder = builder.nanoseconds(num),
            _           => return Err(ClimerError::InvalidTimeIdentifier(caps["ident"].to_string())),
        }
    }
    Ok(builder.build())
}

fn parse_time_with_format(time: &str, format: &str) -> ClimerResult<Time> {
    unimplemented!()
}

#[cfg(test)]
mod tests;
