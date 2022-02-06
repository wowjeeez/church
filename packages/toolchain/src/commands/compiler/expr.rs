use lazy_static::lazy_static;
use regex::Regex;
use crate::commands::compiler::cmd::SourceFile;
use colored::Colorize;
lazy_static! {
    static ref EXPR_RE: Regex = Regex::new(r"//s*@church").unwrap();
    static ref INNER_RE: Regex = Regex::new(r"@church-.+").unwrap();
}

pub struct FoundExpr<'t> {
    file: &'t SourceFile,
    line: usize,
    raw_expr: String,
    parsed_expr: String
}
impl<'t> FoundExpr<'t> {
    pub fn get_line(self: &'t FoundExpr<'t>) -> String {
        self.file.content.lines().nth(self.line).unwrap().to_string()
    }
    pub fn get_line_by_idx(self: &'t FoundExpr<'t>, ln: usize) -> Option<String> {
        let ln = self.file.content.lines().nth(ln);
        return if ln.is_some() {
            Some(ln.unwrap().to_string())
        } else {
            None
        }
    }
    pub fn get_file(self: &'t FoundExpr<'t>) -> &'t SourceFile {
        self.file
    }

}


pub fn parse_expr_in_src(file: &SourceFile) {
    if EXPR_RE.is_match(file.content.as_str()) {
        for (idx, ln) in file.content.lines().enumerate() {
            if ln.trim().starts_with("//") {
                if ln.contains("@church") {
                    let ent = INNER_RE.captures_iter(ln).next();
                    if ent.is_some() {
                        let expr = &ent.unwrap()[0].replace("@church-", "");
                        println!("{}", format!("Found `{}` Church compiler directive in {} at line {}", expr, file.name, idx + 1).yellow());
                    }
                }
            }
        }
    }
}