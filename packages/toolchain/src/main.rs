use std::collections::HashMap;
use std::process::exit;
use lazy_static::lazy_static;
use colored::Colorize;
use crate::cliparser::{CliInp, remap_shorthands, strip_shorthands};
use serde::Deserialize;

mod commands;
mod cliparser;

pub struct Arg {
    pub name: &'static str,
    pub descr: &'static str,
    pub short_hands: Vec<&'static str>
}
pub struct CmdDescr {
    pub name: &'static str,
    pub aliases: Vec<&'static str>,
    pub arguments: Vec<Arg>,
    pub handler: fn (cmd: &CmdDescr, args: &CliInp) -> (),
    pub descr: &'static str
}

impl CmdDescr {
    pub fn shorthands_as_hash(self: &CmdDescr) -> HashMap<String, String> {
        let mut hm = HashMap::new();
        for arg in &self.arguments {
            for shorthand in &arg.short_hands {
                hm.insert(shorthand.to_string(), arg.name.to_string());
            }
        }
        return hm;
    }
}

lazy_static! {
 static ref COMMANDS: [CmdDescr; 1] = [CmdDescr {
    name: "compile",
    aliases: vec!["c"],
    arguments: vec![Arg {name: "config", short_hands: vec!["c"], descr: "The path to church.json"}],
    handler: commands::compiler::compile,
    descr: "Compile a Church project"
}];
}

fn get_cmd(nm: &String) -> Option<&CmdDescr> {
    for cmd in COMMANDS.iter() {
        if cmd.name == nm.as_str() || cmd.aliases.contains(&nm.as_str()) {
            return Option::Some(cmd)
        }
    }
    return None
}

fn print_commands() {
for cmd in COMMANDS.iter() {
    println!("{}", format!("\t{} - Type {} to show information about the command.", cmd.name.yellow(), format!("{} --help", cmd.name).yellow()))
}
}
#[derive(Deserialize)]
pub struct Config {
    src: String,
    entry: String,
    context: String
}

pub fn parse_config(p: String) -> Config {
    let pth = std::path::Path::new(&p);
    if !pth.exists() {
        println!("{}", format!("{} doesn't exist", p).red());
        exit(0)
    }
    let file = std::fs::read_to_string(pth);
    if file.is_err() {
        println!("{}", file.err().unwrap().to_string().red());
        exit(0)
    }
    let parsed_struct: Config = serde_json::from_str(file.unwrap().as_str()).unwrap_or_else(|err| {
        println!("{}", err.to_string().red());
        exit(0)
    });
    parsed_struct
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1);
    if cmd.is_none() {
        println!("{}", "Please specify a command!".red());
        exit(0);
    }
    let obtained_cmd = get_cmd(cmd.unwrap());
    if obtained_cmd.is_none() {
        println!("{} is not a valid command.", cmd.unwrap().yellow());
        println!("Valid commands are:");
        print_commands();
        exit(0)
    }
    let remapped_args = remap_shorthands(&args, obtained_cmd.unwrap().shorthands_as_hash());
    let parsed_args = CliInp::from_vec(strip_shorthands(&remapped_args));
    (obtained_cmd.unwrap().handler)(&obtained_cmd.unwrap(), &parsed_args);
}
