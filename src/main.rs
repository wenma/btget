

#[macro_use]
extern crate clap;
use clap::App;


mod parser;
use parser::TorrentContent;


fn main() {
   let yaml = load_yaml!("../args.yaml");
   let matches = App::from_yaml(yaml).get_matches();

   if matches.is_present("analysis") {
        let content = parser::get_content(matches.value_of("FILE").unwrap()).unwrap();
        let first_token: u8 = content[0];

        let mut torrent: TorrentContent = TorrentContent::new(content);
   		let res = torrent.decode_func(first_token)(&mut torrent);

   		println!("{:?}", res);
   }

}




