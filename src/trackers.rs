
use std::collections::HashMap;


#[derive(Debug)]
struct TrackerParams {
	hash: String,
	peer_id: String,
	upload: usize,
	download: usize,
	left: usize,
	port: u32
}


#[derive(Debug)]
struct TrackerRequest {
	announce: String,
	params: TrackerParams
}

impl TrackerRequest {
	fn new(announce: String, params: TrackerParams) -> Self {
		TrackerRequest {
			announce: announce,
			params: params
		}
	}

	fn do_request(&self) {

	}

}


#[derive(Debug)]
struct TrackerResponse {
	peer_ip: String,
	peer_port: u32,
	interval: u32
}