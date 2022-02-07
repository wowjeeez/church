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
}

pub struct FoundExpr<'t> {
    file: SourceFile<'t>,
    line: usize,
    raw_expr: String,
    parsed_expr: String,
    char_idx: usize
}
impl<'t> FoundExpr<'t> {
    pub fn new(file: SourceFile, line: usize, raw_expr: String, parsed_expr: String, char_idx: usize) -> FoundExpr {
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
        &self.file
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

    pub fn get_after_char_idx(self: &'t FoundExpr<'t>) -> String {
        self.file.content[self.char_idx..].to_string().trim().to_string()
    }

}


pub fn parse_expr_in_src<'t>(file: &'t mut SourceFile<'t>) -> Vec<FoundExpr<'t>> {
    let mut exprs: Vec<FoundExpr<'t>> = vec![];
    let mut last_char_idx: usize = 0;
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
    exprs
}

struct PackedExprHandlerRef {
    handler_ref: Arc<Box<ExprHandler>>,
}

impl PackedExprHandlerRef {
    pub fn call_handler(self: &PackedExprHandlerRef, curr: &'static usize, mut expr: FoundExpr<'static>, exprs: &'static Vec<FoundExpr<'static>>) {
        let get_next = Box::new(|| exprs.get(curr.clone() + 1));
        let get_at = Box::new(|idx: usize| exprs.get(idx));
        (self.handler_ref)(&mut expr, get_next, get_at, curr.to_owned());
    }
    pub fn pack(handler_ref: ExprHandler) -> PackedExprHandlerRef {
        PackedExprHandlerRef {handler_ref: Arc::new(Box::new(handler_ref))}
    }
}

type GetAt = Box<dyn Fn(usize) -> Option<&'static FoundExpr<'static>>>;
type GetNext = Box<dyn Fn() -> Option<&'static FoundExpr<'static>>>;
type ExprHandler = Box<dyn Fn(&mut FoundExpr, GetNext, GetAt, usize) -> () + 'static + Send + Sync>;

pub fn register_expr_handler<F>(expr_name: &str, handler: F)
where
    F: Fn(&mut FoundExpr, GetNext, GetAt, usize) -> () + 'static + Send + Sync
{
    let mut mtx = HANDLERS.lock().unwrap();
    let address = format!("{:p}", &handler);
    println!("[DEBUG]: Registered expression handler for {} (ref: {})", expr_name, address);
    mtx.insert(expr_name.to_string(), PackedExprHandlerRef::pack(Box::new(handler)));
}