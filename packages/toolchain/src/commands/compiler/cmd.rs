use std::fmt::Write;
use std::fs;
use std::path::PathBuf;
use std::process::exit;
use crate::{CliInp, CmdDescr, Config, parse_config};
use colored::Colorize;
use crate::commands::compiler::expr::{FoundExpr, parse_expr_in_src};



#[derive(Debug, Clone)]
pub struct SourceFile {
    pub name: String,
    pub path: PathBuf,
    pub rel_path: PathBuf,
    pub content: String,
    pub config: Config,
}
impl SourceFile {
    pub fn write_new(self: &mut SourceFile, text: String) {
        self.content = text
    }
    pub fn write_after(self: &mut SourceFile, after: usize, str: String) {
        let before = &self.content[..after];
        let done = vec![before, str.as_str()];
        self.content = done.join("");
    }
    pub fn write_between(self: &mut SourceFile, start: usize, end: usize, str: String) {
        let str_start = &self.content[..start];
        let str_end = &self.content[end..];
        let res = vec![str_start, str.as_str(), str_end];
        self.content = res.join("");
    }
}

fn parse_entry(cf: Config) {
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
    let src = SourceFile { name: cf.entry.clone(), path: entry_file, content: entry_f.unwrap(), config: cf.clone(), rel_path: std::path::PathBuf::from(&cf.src).join(&cf.entry) };
    parse_expr_in_src(src);
}

pub fn compile(cmd: &CmdDescr, args: &CliInp) {
    let cnf_p = args.get_string_val("config").unwrap_or("church.json".to_string());
    let cf = parse_config(cnf_p);
    println!("{}", format!("Compiling: {}", cf.project.underline()).green());
    parse_entry(cf.clone());
}