use std::fs;
use std::path::PathBuf;
use std::process::exit;
use crate::{CliInp, CmdDescr, parse_config};
use colored::Colorize;
use crate::commands::compiler::expr::parse_expr_in_src;

#[derive(Debug)]
pub struct SourceFile {
    pub name: String,
    pub path: PathBuf,
    pub content: String
}

pub fn compile(cmd: &CmdDescr, args: &CliInp) {
    let cnf_p = args.get_string_val("config").unwrap_or("church.json".to_string());
    let cf = &parse_config(cnf_p);
    println!("{}", format!("Compiling: {}", cf.project).green());
    let mut entry_file = std::path::PathBuf::new();
    entry_file.push(std::env::current_dir().unwrap());
    entry_file.push(&cf.src);
    entry_file.push(&cf.entry);
    if !entry_file.exists() {
        println!("{}", format!("Entry point at {} doesn't exist.", entry_file.to_str().unwrap()).red());
        exit(0);
    }
    let entry_f = fs::read_to_string(&entry_file);
    if entry_f.is_err() {
            println!("{}", entry_f.err().unwrap().to_string().red());
            exit(0)
    }
    let src = SourceFile { name: cf.entry.clone(), path: entry_file, content: entry_f.unwrap() };
    parse_expr_in_src(&src);

}