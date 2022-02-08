use lazy_static::lazy_static;
use regex::Regex;
use crate::commands::compiler::cmd::SourceFile;
use colored::Colorize;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::fmt::Write;
use crate::fmt_pth;

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

lazy_static! {
    static ref EXPR_RE: Regex = Regex::new(r"//s*@church").unwrap();
    static ref INNER_RE: Regex = Regex::new(r"@church-.+").unwrap();
}



lazy_static! {
    static ref HANDLERS: Mutex<HashMap<String, PackedExprHandlerRef>> = Mutex::new(HashMap::new());
    static ref EXPRS: Mutex<Vec<FoundExpr>> = Mutex::new(Vec::new());
    static ref VALID_EXPRS: Mutex<Vec<String>> = Mutex::new(Vec::new());
}
#[derive(Clone, Debug)]
pub struct FoundExpr {
    pub file: SourceFile,
    pub line: usize,
    raw_expr: String,
    parsed_expr: String,
    char_idx: usize,
    exprs_in_f: usize
}
impl FoundExpr {
    pub fn new(file: SourceFile, line: usize, raw_expr: String, parsed_expr: String, char_idx: usize, exprs_in_f: usize) -> FoundExpr {
        FoundExpr {
            file,
            line,
            raw_expr,
            parsed_expr,
            char_idx,
            exprs_in_f
        }
    }
    pub fn get_line(self: &FoundExpr) -> String {
        self.file.content.lines().nth(self.line).unwrap().to_string() + LINE_ENDING
    }
    pub fn get_line_by_idx(self: &FoundExpr, ln: usize) -> Option<String> {
        let ln = self.file.content.lines().nth(ln);
        return if ln.is_some() {
            Some(ln.unwrap().to_string() + LINE_ENDING)
        } else {
            None
        }
    }
    pub fn get_file(self: &FoundExpr) -> &SourceFile {&self.file}
    pub fn raw_expr(self: &FoundExpr) -> &String {
        &self.raw_expr
    }
    pub fn expr(self: &FoundExpr) -> &String {
        &self.parsed_expr
    }
    pub fn get_src_after_expr(self: &FoundExpr) -> String {
        let res: Vec<String> = self.file.content.lines().map(|l| l.to_string() + LINE_ENDING).collect();
        let s = &res[self.line + 1..].join("");
        s.to_owned().to_string()
    }
    pub fn get_start_char_idx(self: &FoundExpr) -> usize {
        self.char_idx
    }

    pub fn get_after_char_idx(self: &FoundExpr) -> String {
        self.file.content[self.char_idx..].to_string()
    }
    pub fn get_line_term(self: &FoundExpr) -> &'static str {
        LINE_ENDING
    }
    pub fn __set_src(self: &mut FoundExpr, file: SourceFile) {
        self.file = file;
    }
    pub fn __set_found(self: &mut FoundExpr, to: usize) {
        self.exprs_in_f = to
    }
    pub fn get_total_in_file(self: &FoundExpr) -> usize {
        self.exprs_in_f
    }
}


pub fn parse_expr_in_src(file: SourceFile) -> SourceFile {
    let mut exprs = EXPRS.lock().unwrap();
    let mut curr_parsed = std::string::String::new();
    if EXPR_RE.is_match(file.content.as_str()) {
        for (idx, ln) in file.content.lines().enumerate() {
            if ln.trim().starts_with("//") {
                if ln.contains("@church") {
                    let ent = INNER_RE.captures_iter(ln).next();
                    if ent.is_some() {
                        let expr = ent.as_ref().unwrap()[0].replace("@church-", "");
                        {
                            println!("{}", format!("Found {} Church compiler directive in {} at line {}", expr.bold().underline(), fmt_pth(&file.rel_path).bold().underline(), idx + 1).yellow());
                        }
                        let parsed = FoundExpr::new(file.clone(), idx, ent.unwrap()[0].to_string(), expr, curr_parsed.len(), 0);
                        exprs.push(parsed);
                    }
                }
            }
            curr_parsed.write_str(ln);
            curr_parsed.write_str(LINE_ENDING);
        }
    }
    let mut mtx = HANDLERS.lock().unwrap();
    let cl_exprs = exprs.to_vec();
    let ex_len = exprs.len();
    drop(exprs); //drop mutex lock to release the thread
    let valid = VALID_EXPRS.lock().unwrap();
    let mut source_file = Some(file);
    for (idx, mut exp) in cl_exprs.into_iter().enumerate() {
        let hndlr = mtx.get(exp.expr());
        if !valid.contains(exp.expr()) {
            println!("{}", format!("{} is not a valid expression!", exp.expr().bold()).red());
        }
        if hndlr.is_some() {
            exp.__set_src(source_file.unwrap());
            exp.__set_found(ex_len);
           let src = hndlr.unwrap().call_handler(idx, exp);
            source_file = Some(src);
            todo!("Re-calculate start indexes on each source file change");
        }
    }
    drop(valid);
    source_file.unwrap()
}

struct PackedExprHandlerRef {
    handler_ref: Arc<Box<ExprHandler>>,
}

impl PackedExprHandlerRef {
    pub fn call_handler(self: &PackedExprHandlerRef, curr: usize, mut expr: FoundExpr) -> SourceFile {
        let mut vec = EXPRS.lock().unwrap();
        let mut next_c = vec.get(curr + 1).cloned();
        drop(vec); //drop mutex lock to not distrupt closure exec
        let get_at = Box::new(|idx: usize| EXPRS.lock().unwrap().get(idx).cloned());
        (self.handler_ref)(&mut expr, next_c.as_mut(), get_at, curr.to_owned());
        return expr.file
    }
    pub fn pack_boxed(handler_ref: ExprHandler) -> PackedExprHandlerRef {
        PackedExprHandlerRef {handler_ref: Arc::new(Box::new(handler_ref))}
    }
}

type GetAt<'t> = Box<dyn Fn(usize) -> Option<FoundExpr>>;
type ExprHandler = Box<dyn Fn(&mut FoundExpr, Option<&mut FoundExpr>, GetAt, usize) -> () + 'static + Send + Sync>;

pub fn register_expr_handler<F>(expr_name: &str, handler: F)
where
    F: Fn(&mut FoundExpr, Option<&mut FoundExpr>, GetAt, usize) -> () + 'static + Send + Sync
{
    let mut mtx = HANDLERS.lock().unwrap();
    mtx.insert(expr_name.to_string(), PackedExprHandlerRef::pack_boxed(Box::new(handler)));
    VALID_EXPRS.lock().unwrap().push(expr_name.to_string());
}