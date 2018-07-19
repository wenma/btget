

#[macro_use]
extern crate clap;
use clap::App;

#[macro_use]
extern crate prettytable;
extern crate number_prefix;
extern crate encoding;
extern crate sha1;

mod parser;
mod encode;
mod trackers;

use parser::{FileContent, TorrentContent, Value};
use encode::{to_url_encode, hex_to_binary, info_hash};


fn main() {
    let yaml = load_yaml!("../args.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    let content = parser::get_content(matches.value_of("FILE").unwrap()).unwrap();
    let first_token: u8 = content[0];

    println!("\n种子特征值: {}", info_hash(&content));

    let mut torrent: TorrentContent = TorrentContent::new(content);
    let res = torrent.decode_func(first_token)(&mut torrent);

    if matches.is_present("analysis") {
        let mut files = Vec::<FileContent>::new();
        Value::contents(res, &mut files);
        FileContent::pprint(&files);

    } else {

        println!("Those features are Not Completed yet !");

        println!("{:?}", res);

        // println!("{:?}", res.get_trackers());

        // println!("{:?}", to_url_encode(&[104, 101, 108, 108, 111][..]));

        // println!("{:?}", hex_to_binary("8d450423d183764da01e30118e480df228f19450".to_string()));

        // println!("{:?}", to_url_encode(&hex_to_binary("8d450423d183764da01e30118e480df228f19450".to_string()).unwrap()[..]));   

        // curl 'http://tracker1.itzmx.com:8080/announce?info_hash=%8D%45%04%23%D1%83%76%4D%A0%1E%30%11%8E%48%0D%F2%28%F1%94%50&peer_id=%8D%45%04%23%D1%83%76%4D%A0%1E%30%11%8E%48%0D%F2%28%F1%94%51&uploaded=0&downloade=0&left=1518815706&port=6300' -XGET     
    }

}



