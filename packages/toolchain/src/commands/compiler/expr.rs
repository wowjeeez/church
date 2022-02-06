use lazy_static::lazy_static;
use regex::Regex;
use crate::commands::compiler::cmd::SourceFile;

lazy_static! {
    static ref EXPR_RE: Regex = Regex::new(r"\/\/\s*@church").unwrap();
    static ref INNER_RE: Regex = Regex::new(r"@church-.+").unwrap();
}

pub fn parse_expr_in_src(file: SourceFile) {
    if EXPR_RE.is_match(file.content.as_str()) {
        for ln in file.content.lines() {
            if ln.trim().starts_with("//") {
                if ln.contains("@church") {
                    let ent = INNER_RE.captures_iter(ln).next();
                    if ent.is_some() {
                        let expr = &ent.unwrap()[0];
                        println!("{}", expr)
                    }
                }
            }
        }
    }
}