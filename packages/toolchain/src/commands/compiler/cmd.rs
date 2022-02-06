use crate::{CliInp, CmdDescr};
pub struct SourceFile {
    pub name: String,
    pub path: String,
    pub content: String
}

pub fn compile(cmd: &CmdDescr, args: &CliInp) {
dbg!(args);
println!("Compiling project...");
    println!("Looking for config file at: {}", args.get_string_val("config").unwrap());

}