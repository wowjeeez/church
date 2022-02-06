use std::collections::HashMap;
use std::ops::Index;
use std::path::PathBuf;

#[derive(Debug)]
pub struct CliInp(Vec<String>);


impl CliInp {
    pub fn from_vec(data: Vec<String>) -> CliInp {
        CliInp(data)
    }
    pub fn get_bool_flag(self: &CliInp, flag: &str) -> bool {
        for (idx, f) in self.0.iter().enumerate() {
            if *f == format!("--{}", flag) {
                if self.0.get(idx + 1).is_some() {
                    let val = self.0.get(idx + 1).unwrap();
                    if val.starts_with("--") {
                        return true
                    }
                }
            }
        }
        return false
    }
    pub fn get_string_val(self: &CliInp, key: &str) -> Option<String> {
        let mut start_idx: i64 = -1;
        for (idx, flag) in self.0.iter().enumerate() {
            if *flag == format!("--{}", key) {
                start_idx = idx as i64 + 1;
                break;
            }
        }
        if start_idx == -1 {
            return None;
        }
        let casted = start_idx as usize;
        let sliced = &self.0[casted..];
        let mut res_vec: Vec<String> = vec![];
        for val in sliced.iter() {
            if val.starts_with("--") {
                break;
            } else {
                res_vec.push(val.clone());
            }
        }
        return Some(res_vec.join(" "))
    }
    pub fn get_raw_inp_after_cmd(self: &CliInp) -> Vec<String> {
        return self.0[2..].to_vec()
    }
    pub fn get_bin_pth(self: &CliInp) -> PathBuf {
        let mut p = PathBuf::new();
        p.push(std::env::current_dir().unwrap());
        p.push(self.0.get(0).unwrap());
        p
    }
}

pub fn remap_shorthands(vec: &Vec<String>, flags: HashMap<String, String>) -> Vec<String> {
    let mut remapped_vec = vec.clone();
    for (idx, flag) in vec.iter().enumerate() {
        let sh = flags.get(flag.replace("-", "").as_str());
        if sh.is_some() {
            remapped_vec[idx] = format!("--{}", sh.unwrap().clone());
        }
    }
    return remapped_vec
}

pub fn strip_shorthands(vec: &Vec<String>) -> Vec<String> {
    let mut remapped_vec = vec.clone();
    for (idx, flag) in vec.iter().enumerate() {
       let char = flag.chars().nth(1).unwrap().to_string();
        if char != "-" && flag.starts_with("-") {
            remapped_vec.remove(idx);
        }
    }
    return remapped_vec
}