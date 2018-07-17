
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