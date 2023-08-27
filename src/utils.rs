use std::path::PathBuf;

pub fn is_numeric(s: &str) -> bool {
    s.parse::<u32>().is_ok()
}

pub fn has_valid_extension(path: &PathBuf, ext_type: &str) -> bool {
    if let Some(ext) = path.extension() {
        if ext == ext_type {
            return true;
        }
    }

    false
}

fn pad_string(s: &str, pad_len: u8) -> String {
    let mut s = String::from(s);
    while s.len() < pad_len.into() {
        s = "0".to_owned() + &s;
    }

    s
}

pub fn dec_to_bin(dec: u32, len: Option<u8>) -> String {
    let bin = format!("{:b}", dec);
    if let Some(len) = len {
        if bin.len() < len.into() {
            return pad_string(&bin, len);
        }
    }

    bin
}
