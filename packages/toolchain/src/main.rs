use std::any::Any;
use std::process::exit;
use lazy_static::lazy_static;
use colored::Colorize;


mod commands;
pub struct Arg {
    pub name: &'static str,
    pub descr: &'static str,
    pub short_hands: Vec<&'static str>
}
pub struct CmdDescr {
    pub name: &'static str,
    pub aliases: Vec<&'static str>,
    pub arguments: Vec<Arg>,
    pub handler: fn (cmd: &CmdDescr, args: &Vec<String>) -> (),
    pub descr: &'static str
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
    (obtained_cmd.unwrap().handler)(&obtained_cmd.unwrap(), &args);
}
