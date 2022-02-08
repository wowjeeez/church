use std::fs;
use std::path::{PathBuf};
use std::process::exit;
use crate::{CliInp, CmdDescr, Config, fmt_pth, parse_config};
use colored::Colorize;
use crate::commands::compiler::log_repl::repl_logs_with_ctx;


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
}

fn handle_file(cf: &Config, fp: PathBuf) {
    let f_c = fs::read_to_string(&fp).unwrap();
    let pwd = std::env::current_dir().unwrap();
    let rel = fmt_pth(&PathBuf::from(fp.to_str().unwrap().replace(pwd.to_str().unwrap(), "")));
    let modif = repl_logs_with_ctx(&cf.context, !cf.javascript, f_c, rel);
    let r#final = format!(r#"exports("__CHRCH__GET_TYPE", () => "{}")"#, cf.r#type);
    fs::write(&fp, r#final + "\n" + &*modif);
}

fn parse_dirs(cf: Config) {
    let src_dir = std::env::current_dir().unwrap().join(&cf.src);
    if !src_dir.exists() {
        println!("{}", format!("{} doesn't exist.", src_dir.to_str().unwrap()).red());
        exit(0);
    }
    let dirs = walkdir::WalkDir::new(src_dir);
    let ts_proj = !cf.javascript;
    for f in dirs.into_iter().filter(|w| w.is_ok()).map(|w| w.unwrap()) {
        if f.metadata().unwrap().is_file() {
            if ts_proj && f.file_name().to_str().unwrap().ends_with(".js") {
                println!("{}", format!("{} files found in a TypeScript project. This is bad, add: {} to your church.json.", ".js".bold(), r#""javascript: true""#.bold()).red());
                exit(0);
            } else if !ts_proj && f.file_name().to_str().unwrap().ends_with(".ts") {
                println!("{}", format!("{} files found in a JavaScript project. This is bad, add: {} to your church.json.", ".ts".bold(), r#""javascript: false""#.bold()).red());
                exit(0);
            }
            handle_file(&cf, f.path().to_path_buf());
        }
    }

}


pub fn compile(cmd: &CmdDescr, args: &CliInp) {
    let cnf_p = args.get_string_val("config").unwrap_or("church.json".to_string());
    let cf = parse_config(cnf_p);
    println!("{}", format!("Compiling: {}", cf.project.underline()).green());
    //parse_entry(cf.clone());
    parse_dirs(cf.clone());
}