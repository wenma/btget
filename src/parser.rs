
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;

use prettytable::Table;
use prettytable::format;
use number_prefix::{binary_prefix, Standalone, Prefixed};


pub fn get_content(path: &str) -> ::std::io::Result<Vec<u8>> { 
    let filename = absolute_path(path).unwrap();
    let mut file = File::open(&filename)?;
    let mut content: Vec<u8> = Vec::new();
    file.read_to_end(&mut content)?;
    Ok(content)
}


fn absolute_path(path: &str) -> Option<String> {
    let p = Path::new(path);
    match p.canonicalize() {
        Ok(buf) => Some(String::from(buf.to_str().unwrap())),
        Err(_) => None
    }
}


fn to_sha1_hex(slice: &[u8]) -> String {
    let mut ret: String = String::new();
    for s in slice {
        ret += format!("{:02x}", s).as_str();
    }
    ret
}


#[derive(Debug)]
pub struct FileContent {
    path: String,
    length: u64,
    filehash: String
}


impl FileContent {
    pub fn pprint(files: &Vec<FileContent>) {
        let mut table = Table::new();

        table.add_row(row![bFg -> "文件大小", bFg -> "文件名", bFg -> "hash值"]);
        for file in files {
            let length = match binary_prefix(file.length as f32) {
                Prefixed(prefix, n) => format!("{:.2} {}B", n, prefix),
                Standalone(bytes) => format!("{} bytes", bytes)
            };
            table.add_row(row![length, file.path, file.filehash]);
        }

        table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
        table.printstd();
        println!("");
    }
}


#[derive(Debug)]
pub enum Value {
    Int(u64),
    Strings(String),
    List(Vec<Value>),
    Dict(HashMap<String, Value>)
}


impl Value {
    fn get_string(&self) -> String {
        match self {
            Value::List(list) => {
                let mut target = String::new();
                for l in list {
                    target += &match l {
                        Value::Strings(s) => s[..].to_string(),
                        _ => panic!("Error when parse string!")
                    };
                    target += "\n";
                }
                target
            }
            Value::Strings(s) => s[..].to_string(),
            _ => panic!("Error when parse string!")
        }
    }

    fn get_int(&self) -> u64 {
        match self {
            Value::Int(n) => *n,
            _ => panic!("Error when parse int!")
        }
    }

    pub fn contents(res: Value, files: &mut Vec<FileContent>) {
        match res {
            Value::List(list) => {
                for l in list {
                    Value::contents(l, files);
                }
            },
            Value::Dict(dict) => {
                if dict.contains_key("path") || dict.contains_key("path.utf-8") {
                    let path = dict.get("path".into())
                                   .unwrap().get_string();

                    if !path.starts_with("____") {
                        files.push(FileContent{
                            path: path,
                            length: dict.get("length".into())
                                        .unwrap_or(&Value::Int(0)).get_int(),

                            filehash: dict.get("filehash".into())
                                          .unwrap_or(&Value::Strings("".into())).get_string()
                        });
                    }   
                }
                
                for (_, v) in dict {
                    Value::contents(v, files);
                }
            },
            _ => {}
        }
    }
}


#[derive(Debug)]
pub struct TorrentContent {
    content: Vec<u8>,
    index: usize
}


impl TorrentContent {
    pub fn new(content: Vec<u8>) -> Self {
        TorrentContent {
            content: content,
            index: 0
        }
    }

    pub fn decode_func(&self, token: u8) -> fn(&mut TorrentContent) -> Value {
        match token {
            b'l' => TorrentContent::decode_list,
            b'd' => TorrentContent::decode_dict,
            b'i' => TorrentContent::decode_int,
            b'0' ... b'9' => TorrentContent::decode_string,
            _ => panic!("Unknown token...")
        }
    }

    fn decode_int(&mut self) -> Value {
        self.index += 1;

        let mut newf = self.index;
        while self.content[newf] != b'e' {
            newf += 1;
        }

        let sn = String::from_utf8(self.content[self.index .. newf].to_vec()).unwrap();
        let n = sn.parse::<u64>().unwrap();

        if self.content[self.index] == b'-' {
            if self.content[self.index + 1] == b'0' {
                panic!("Error when decode int!");
            }
        } else if self.content[self.index] == b'0' && newf != self.index + 1 {
            panic!("Error when decode int!");
        }
        
        self.index = newf + 1;
        Value::Int(n)

    }

    fn decode_string(&mut self) -> Value {

        let mut colon = self.index;
        while self.content[colon] != b':' {
            colon += 1;
        }

        let sn = String::from_utf8(self.content[self.index .. colon].to_vec()).unwrap();
        let n = sn.parse::<u64>().unwrap() as usize;

        if self.content[self.index] == b'0' && colon != self.index + 1 {
            panic!("Error when decode string!");
        }

        colon += 1;
        self.index = colon + n;

        match String::from_utf8(self.content[colon .. self.index].to_vec()) {
            Ok(s) => Value::Strings(s),
            Err(_) => Value::Strings(to_sha1_hex(&self.content[colon .. self.index]))
        }
    }

    fn decode_list(&mut self) -> Value {
        let mut data: Vec<Value> = Vec::new();
        self.index += 1;

        while self.content[self.index] != b'e' {
            let f = self.decode_func(self.content[self.index])(self);
            data.push(f);
        }

        self.index += 1;
        Value::List(data)
    }

    fn decode_dict(&mut self) -> Value {
        let mut data = HashMap::<String, Value>::new();
        self.index += 1;

        while self.content[self.index] != b'e' {
            let v = self.decode_string();
            let f = self.decode_func(self.content[self.index])(self);
            match v {
                Value::Strings(k) => { data.insert(k, f); },
                _ => {},
            }
        }

        self.index += 1;
        Value::Dict(data)
    }
}
