

#[macro_use]
extern crate clap;
use clap::App;

#[macro_use] 
extern crate prettytable;
extern crate number_prefix;

mod parser;
use parser::{FileContent, TorrentContent, Value};


fn main() {
   let yaml = load_yaml!("../args.yaml");
   let matches = App::from_yaml(yaml).get_matches();

   if matches.is_present("analysis") {
        let content = parser::get_content(matches.value_of("FILE").unwrap()).unwrap();
        let first_token: u8 = content[0];

        let mut torrent: TorrentContent = TorrentContent::new(content);
        let res = torrent.decode_func(first_token)(&mut torrent);

        let mut files = Vec::<FileContent>::new();
        Value::contents(res, &mut files);

        FileContent::pprint(&files);        

   } else {
        println!("Those features are Not Completed yet !");
   }

}




