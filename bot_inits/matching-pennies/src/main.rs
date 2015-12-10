extern crate zmq;
use std::thread;

extern crate rmp_serialize as msgpack;
extern crate rustc_serialize;

use rustc_serialize::{Encodable, Decodable};
use msgpack::{Encoder, Decoder};

fn main() {

    #[derive(RustcEncodable, RustcDecodable, PartialEq, Debug)]
    struct Pick{
        p: String
    }

    let mut ctx = zmq::Context::new();

    let mut responder = ctx.socket(zmq::REP).unwrap();

    assert!(responder.bind("tcp://*:5555").is_ok());

    //let mut msg: Vec<u8> = Vec::new();

    responder.set_rcvhwm(1).unwrap();

    loop {

        println!("Receiving...");

        let byte_msg = responder.recv_bytes(0).unwrap();

        println!("Received {:?}", byte_msg);

        let mut decoder = Decoder::new(&byte_msg[..]);
        let res: Pick = Decodable::decode(&mut decoder).ok().unwrap();
        
        println!("Received msgpack {:?}", res);

        responder.send_str("World", 0).unwrap();
        
        thread::sleep_ms(1000);

    }

}
