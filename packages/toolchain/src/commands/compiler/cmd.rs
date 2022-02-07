use std::fs;
use std::path::PathBuf;
use std::process::exit;
use crate::{CliInp, CmdDescr, Config, parse_config};
use colored::Colorize;
use crate::commands::compiler::expr::{FoundExpr, parse_expr_in_src};



#[derive(Debug, Clone)]
pub struct SourceFile<'t> {
    pub name: String,
    pub path: PathBuf,
    pub rel_path: PathBuf,
    pub content: String,
    pub config: &'t Config
}
impl<'t> SourceFile<'t> {
    pub fn write_new(self: &'t mut SourceFile<'t>, text: String) {
        self.content = text
    }
    pub fn write(self: &'t SourceFile<'t>) {

    }
}

fn parse_entry(cf: &Config) {
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
    let mut src = SourceFile { name: cf.entry.clone(), path: entry_file, content: entry_f.unwrap(), config: &cf, rel_path: std::path::PathBuf::from(&cf.src).join(&cf.entry) };
    let exprs = parse_expr_in_src(&mut src);
}

pub fn compile(cmd: &CmdDescr, args: &CliInp) {
    let cnf_p = args.get_string_val("config").unwrap_or("church.json".to_string());
    let cf = &parse_config(cnf_p);
    println!("{}", format!("Compiling: {}", cf.project.underline()).green());
    parse_entry(&cf);
}