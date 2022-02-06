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
    parsed_expr: String,
    char_idx: usize
}
impl<'t> FoundExpr<'t> {
    pub fn new(file: &SourceFile, line: usize, raw_expr: String, parsed_expr: String, char_idx: usize) -> FoundExpr {
        FoundExpr {
            file,
            line,
            raw_expr,
            parsed_expr,
            char_idx
        }
    }
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
    pub fn raw_expr(self: &'t FoundExpr<'t>) -> &'t String {
        &self.raw_expr
    }
    pub fn expr(self: &'t FoundExpr<'t>) -> &'t String {
        &self.parsed_expr
    }
    pub fn get_src_after_expr(self: &'t FoundExpr<'t>) -> String {
        let res: Vec<String> = self.file.content.lines().map(|l| l.to_string()).collect();
        let s = &res[self.line + 1..].join("");
        s.to_owned().trim().to_string()
    }
    pub fn get_start_char_idx(self: &'t FoundExpr<'t>) -> usize {
        self.char_idx
    }
    pub fn get_full(self: &'t FoundExpr<'t>) -> String {
        self.file.content[self.char_idx..].to_string().trim().to_string()
    }

}


pub fn parse_expr_in_src<'t>(file: &'t SourceFile) -> Vec<FoundExpr<'t>> {
    let mut exprs: Vec<FoundExpr<'t>> = vec![];
    let mut last_char_idx: usize = 0;
    if EXPR_RE.is_match(file.content.as_str()) {
        for (idx, ln) in file.content.lines().enumerate() {
            if ln.trim().starts_with("//") {
                if ln.contains("@church") {
                    let ent = INNER_RE.captures_iter(ln).next();
                    if ent.is_some() {
                        let expr = ent.as_ref().unwrap()[0].replace("@church-", "");
                        println!("{}", format!("Found `{}` Church compiler directive in {} at line {}", expr, file.name, idx + 1).yellow());
                        let parsed = FoundExpr::new(file, idx, ent.unwrap()[0].to_string(), expr.clone(), last_char_idx);
                        exprs.push(parsed);
                    }
                }
            }
            last_char_idx += ln.len();
        }
    }
    return exprs
}