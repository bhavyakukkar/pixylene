use crate::ui::{UiFn};

use clap::Parser;
use serde::{Serialize, Deserialize};


pub fn parse_cmd(string: &str) -> Result<UiFn, String> {
    #[derive(Debug, Parser)]
    struct Container {
        #[command(subcommand)]
        u: UiFn
    }
    Container::try_parse_from(
        shlex::split(&(": ".to_owned() + string)).ok_or("malformed input".to_owned())?)
        .map(|container: Container| container.u)
        .map_err(|err| err.to_string())
}

pub fn _parse_toml(string: &str) -> Result<UiFn, toml::de::Error> {
    #[derive(Deserialize)]
    struct Container {
        #[allow(dead_code)]
        u: UiFn,
    }

    let mut string = String::from(string);
    if string.find("{").is_none() {
        string.insert(0, '"');
        string.push('"');
    } else {
        string.insert(0, '{');
        string.push('}');
    }

    toml::from_str(&format!("u = {}", string))
        .map(|container: Container| container.u)
}

pub fn deparse(uifns: &Vec<UiFn>) -> String {
    #[derive(Serialize)]
    struct Container {
        u: UiFn,
    }

    "[".to_owned() +
    &uifns.iter()
        .map(|uifn| {
            let mut value = String::new();
            Container{ u: uifn.clone() }.serialize(toml::ser::ValueSerializer::new(&mut value)).unwrap_or(());
            value[6..(value.len() - 2)].to_owned()
        })
        .reduce(|a, b| a + ", " + &b).unwrap_or(" ".to_owned()) +
    "]"
    //"[".to_owned() + &uifns.iter()
    //    .map(|uifn| toml::to_string(&Container { u: uifn.clone() })
    //        .map(|ser| ser/*.replace("\n", " ")[4..(ser.len()-1)].to_owned()*/)
    //        .expect("Failed to serialize UiFn"))
    //    .reduce(|a, b| a + ", " + &b).unwrap_or(" ".to_owned()) + "]"
}
