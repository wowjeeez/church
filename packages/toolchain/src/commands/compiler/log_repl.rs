const LOG_FN_FIGS: [&str; 5] = ["debug(", "info(", "silly(", "trace(", "error("];

use regex::Regex;
use colored::Colorize;
fn does_file_define_custom_log_impl(r#for: &String, src: &String) -> bool {
    let imp_rep_nodef = Regex::new(format!(r"import\s+\{{\s*{}\s*\}}", r#for).as_str()).unwrap();
    let imp_rep_def = Regex::new(format!(r"import\s+{}\s+", r#for).as_str()).unwrap();
    let decl_var = Regex::new(format!(r"(const|var|let)\s+{}(\s+|=)", r#for).as_str()).unwrap(); //this checks for cjs imports and arrow func declarations too
    let func_decl = Regex::new(format!(r"function\s+{}", r#for).as_str()).unwrap();
    let var_binds = decl_var.is_match(src.as_str());
    let func_decl = func_decl.is_match(src.as_str());
    let import_es = imp_rep_def.is_match(src.as_str()) && imp_rep_nodef.is_match(src.as_str());
    var_binds || func_decl || import_es

}
pub fn repl_logs_with_ctx(context: &String, is_ts: bool, src: String, rel_p: String) -> String {
    let import_st = if is_ts {
        format!(r#"import {{log as __CHRCH_LOG}} from "@church/{}"#, context)
    } else {
        format!(r#"const __CHRCH_LOG = require("@church/{}").log"#, context)
    };
    let mut mod_src = String::from(&src);
    for sig in LOG_FN_FIGS {
        let fn_name = sig.replace("(", "");
        if !does_file_define_custom_log_impl(&fn_name, &mod_src) {
            mod_src = mod_src.replace(sig, format!(r#"__CHRCH_LOG({{level: "{}", location: "{}", async: true}}, "#, fn_name, rel_p).as_str())
        } else {
            println!("{}", format!("{} defines a custom implementation for {}.", rel_p.underline(), format!("{}()", fn_name).bold()).yellow());
        }
    };
    return if mod_src.len() != src.len() {
        format!("{}\n{}", import_st, mod_src)
    } else {
            mod_src
    }
}
