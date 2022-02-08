use crate::{fmt_pth, register_expr_handler};
use colored::Colorize;
use crate::commands::compiler::expr::FoundExpr;

pub fn ignore() {
    register_expr_handler("ignore-prod" ,|ex, get_next, _, _| {
        println!("Handling expression: {}", ex.expr());
        let line_end = ex.get_line_by_idx(ex.line + 1).unwrap();
        let start_idx = ex.get_start_char_idx();
        let to_strip = start_idx + ex.raw_expr().len() + line_end.len() + (ex.get_line_term().len() * 2);
        let file = &mut ex.file;
        println!("{}", format!("Ignoring line: {}", line_end.bold()).yellow());
        file.write_between(start_idx, to_strip, "".to_string());
    });

    register_expr_handler("comment-prod" ,|ex, _, _, _| {
        let line_end = ex.get_line_by_idx(ex.line + 1).unwrap();
        let start_idx = ex.get_start_char_idx();
        let to_strip = start_idx + ex.raw_expr().len() + line_end.len() + (ex.get_line_term().len() * 2);
        let file = &mut ex.file;
        println!("{}", format!("Commenting line: {}", line_end.bold()).yellow());
        file.write_between(start_idx, to_strip, format!("//{}", line_end));
    });
    register_expr_handler("ignore-prod-start", |ex, get_next, get_at, idx| {
        let mut ctr = idx + 1;
        let boundary = ex.get_total_in_file();
        let mut end: Option<FoundExpr> = None;
        loop {
            let next_exp: Option<FoundExpr> = get_at(ctr);
            if next_exp.is_some() {
                let u_w = next_exp.unwrap();
                if u_w.expr() == &"ignore-prod-end".to_string() {
                    end = Some(u_w);
                    break;
                }
            }
            if ctr == boundary {
                break;
            }
            ctr += 1;
        }
        if end.is_none() {
            println!("{}", format!("No closing expression found for {} in {}", ex.expr().bold(), fmt_pth(&ex.file.rel_path).bold()).red())
        } else {
            let end_expr = end.unwrap();
            let end_chr_idx = end_expr.get_start_char_idx() + end_expr.raw_expr().len() + ex.get_line_term().len();
            let start_chr_idx = ex.get_start_char_idx();
            ex.file.write_between(start_chr_idx, end_chr_idx, "".to_string());
            dbg!(&ex.file.content);
            std::fs::write("./out.ts", &ex.file.content);
        }
    });
    register_expr_handler("ignore-prod-end", |_, _, _, _| {});

}