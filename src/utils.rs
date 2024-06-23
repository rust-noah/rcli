use anyhow::Result;
use std::{fs::File, io::Read};

// give a input(-[default] or Path) return a reader, can read_to_end

// 返回一个读取器，可以从输入(-[默认]或路径)读取数据到结尾
pub fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    Ok(reader)
}

/// give a input(-[default] or Path) return a Vec
pub fn get_content(input: &str) -> Result<Vec<u8>> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    Ok(buf)
}
