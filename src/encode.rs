
extern crate sha1;
use std::u8;


pub fn hex_to_binary(hex: String) -> Result<Vec<u8>, String> {
    let mut ret: Vec<u8> = Vec::new();
    let mut index = 0;
    while index < hex.len() {
        if hex.get(index..index + 2).is_none() {
            return Err("Not a valid hex!".to_string());
        }
        match u8::from_str_radix(hex.get(index..index + 2).unwrap(), 16) {
            Ok(i) => ret.push(i),
            Err(_) => return Err("Not a valid hex!".to_string())
        }
        index += 2;
    }
    Ok(ret)
}

pub fn utf8_to_binary(u: String) -> Vec<u8> {
	u.into_bytes()
}

pub fn to_sha1_hex(slice: &[u8]) -> String {
    let mut encode: String = String::new();
    for s in slice {
        encode += format!("{:02x}", s).as_str();
    }
    encode
}

pub fn to_url_encode(slice: &[u8]) -> String {
	let mut encode: String = String::new();
	for s in slice {
        encode += format!("%{:02X}", s).as_str();
    }
    encode
}

pub fn info_hash(content: &Vec<u8>) -> String {
    let mut i = 0;
    let mut stack = Vec::<u8>::new();

    while i < content.len() {
        if &content[i..i + 6] == b"4:info" {

                let mut j = i + 6;
                stack.push(content[j]);
                let first = j;
                j += 1;

                while !stack.is_empty() {
                    match content[j] {
                        b'l' => {
                            stack.push(content[j]);
                            j += 1;
                        },
                        b'd' => {
                            stack.push(content[j]);
                            j += 1;
                        },
                        b'i' => {
                            stack.push(content[j]);
                            while content[j] != b'e' {
                                j += 1;
                            }
                        },
                        b'0'...b'9' => {
                            let mut k = j;
                            while content[k] != b':' {
                                k += 1;
                            }
                            let sn = String::from_utf8(content[j..k].to_vec()).unwrap();
                            let n = sn.parse::<u64>().unwrap() as usize;
                            j = k + 1 + n;
                        },
                        b'e' => {
                            stack.pop();
                            j += 1;
                        }
                        _ => {}
                    }
                }

                let mut m = sha1::Sha1::new();
                match content.get(first..j) {
                    Some(s) => {
                        m.update(s);
                        return m.digest().to_string();
                    },
                    None => String::from("")
                };
        }
        i += 1;
    }
    String::from("")
}
