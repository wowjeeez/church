use lazy_static::lazy_static;
use regex::Regex;
use crate::commands::compiler::cmd::SourceFile;
use colored::Colorize;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

lazy_static! {
    static ref EXPR_RE: Regex = Regex::new(r"//s*@church").unwrap();
    static ref INNER_RE: Regex = Regex::new(r"@church-.+").unwrap();
}



lazy_static! {
    static ref HANDLERS: Mutex<HashMap<String, PackedExprHandlerRef>> = Mutex::new(HashMap::new());
    static ref EXPRS: Mutex<Vec<FoundExpr>> = Mutex::new(Vec::new());
    static ref VALID_EXPRS: Mutex<Vec<String>> = Mutex::new(Vec::new());
}
#[derive(Clone)]
pub struct FoundExpr {
    file: SourceFile,
    line: usize,
    raw_expr: String,
    parsed_expr: String,
    char_idx: usize
}
impl FoundExpr {
    pub fn new(file: SourceFile, line: usize, raw_expr: String, parsed_expr: String, char_idx: usize) -> FoundExpr {
        FoundExpr {
            file,
            line,
            raw_expr,
            parsed_expr,
            char_idx
        }
    }
    pub fn get_line(self: &FoundExpr) -> String {
        self.file.content.lines().nth(self.line).unwrap().to_string()
    }
    pub fn get_line_by_idx(self: &FoundExpr, ln: usize) -> Option<String> {
        let ln = self.file.content.lines().nth(ln);
        return if ln.is_some() {
            Some(ln.unwrap().to_string())
        } else {
            None
        }
    }
    pub fn get_file(self: &FoundExpr) -> &SourceFile {
        &self.file
    }
    pub fn raw_expr(self: &FoundExpr) -> &String {
        &self.raw_expr
    }
    pub fn expr(self: &FoundExpr) -> &String {
        &self.parsed_expr
    }
    pub fn get_src_after_expr(self: &FoundExpr) -> String {
        let res: Vec<String> = self.file.content.lines().map(|l| l.to_string()).collect();
        let s = &res[self.line + 1..].join("");
        s.to_owned().trim().to_string()
    }
    pub fn get_start_char_idx(self: &FoundExpr) -> usize {
        self.char_idx
    }

    pub fn get_after_char_idx(self: &FoundExpr) -> String {
        self.file.content[self.char_idx..].to_string().trim().to_string()
    }

}


pub fn parse_expr_in_src(file: SourceFile) {
    let mut last_char_idx: usize = 0;
    let mut exprs = EXPRS.lock().unwrap();
    if EXPR_RE.is_match(file.content.as_str()) {
        for (idx, ln) in file.content.lines().enumerate() {
            if ln.trim().starts_with("//") {
                if ln.contains("@church") {
                    let ent = INNER_RE.captures_iter(ln).next();
                    if ent.is_some() {
                        let expr = ent.as_ref().unwrap()[0].replace("@church-", "");
                        {
                            println!("{}", format!("Found {} Church compiler directive in {} at line {}", expr.bold().underline(), file.rel_path.to_str().unwrap().replace("\\", "/").bold().underline(), idx + 1).yellow());
                        }
                        let parsed = FoundExpr::new(file.clone(), idx, ent.unwrap()[0].to_string(), expr, last_char_idx);
                        exprs.push(parsed);
                    }
                }
            }
            last_char_idx += ln.len();
        }
    }
    let mut mtx = HANDLERS.lock().unwrap();
    let cl_exprs = exprs.to_vec();
    drop(exprs); //drop mutex lock to release the thread
    let valid = VALID_EXPRS.lock().unwrap();
    for (idx, exp) in cl_exprs.into_iter().enumerate() {
        let hndlr = mtx.get(exp.expr());
        if !valid.contains(exp.expr()) {
            println!("{}", format!("{} is not a valid expression!", exp.expr().bold()).red());
        }
        if hndlr.is_some() {
            hndlr.unwrap().call_handler(idx, exp)
        }
    }
    drop(valid);
}

struct PackedExprHandlerRef {
    handler_ref: Arc<Box<ExprHandler>>,
}

impl PackedExprHandlerRef {
    pub fn call_handler(self: &PackedExprHandlerRef, curr: usize, mut expr: FoundExpr) {
        let vec = EXPRS.lock().unwrap();
        let next_c = vec.get(curr + 1);
        let get_at = Box::new(|idx: usize| {
            let uw = EXPRS.lock().unwrap();
            uw.get(idx).cloned()
        });
        (self.handler_ref)(&mut expr, next_c, get_at, curr.to_owned());
    }
    pub fn pack_boxed(handler_ref: ExprHandler) -> PackedExprHandlerRef {
        PackedExprHandlerRef {handler_ref: Arc::new(Box::new(handler_ref))}
    }
}

type GetAt<'t> = Box<dyn Fn(usize) -> Option<FoundExpr>>;
type ExprHandler = Box<dyn Fn(&mut FoundExpr, Option<&FoundExpr>, GetAt, usize) -> () + 'static + Send + Sync>;

pub fn register_expr_handler<F>(expr_name: &str, handler: F)
where
    F: Fn(&mut FoundExpr, Option<&FoundExpr>, GetAt, usize) -> () + 'static + Send + Sync
{
    let mut mtx = HANDLERS.lock().unwrap();
    let address = format!("{:p}", &handler).bold();
    println!("[DEBUG]: Registered expression handler for {} (ref: {})", expr_name.bold(), address);
    mtx.insert(expr_name.to_string(), PackedExprHandlerRef::pack_boxed(Box::new(handler)));
    VALID_EXPRS.lock().unwrap().push(expr_name.to_string());
}