extern crate zmq;
use std::thread;

extern crate rmp_serialize as msgpack;
extern crate rustc_serialize;

use rustc_serialize::{Encodable, Decodable};
use msgpack::{Encoder, Decoder};

#[derive(RustcEncodable, RustcDecodable, PartialEq, Debug, Clone, Copy)]
struct GameStatus{
    //Round
    r: u64,
    //Pick
    p: char,
    //Score
    s: u64,
    //Status
    t: char 
}

#[derive(RustcEncodable, RustcDecodable, PartialEq, Debug)]
struct Pick{
    p: char 
}

fn main() {    

    let mut ctx = zmq::Context::new();

    let mut responder = ctx.socket(zmq::REP).unwrap();

    assert!(responder.bind("tcp://*:5555").is_ok());

    //let mut msg: Vec<u8> = Vec::new();

    responder.set_rcvhwm(1).unwrap();

    let pick = Pick { p: '0' };

    let mut pick_buf = Vec::with_capacity(30);
    pick.encode(&mut Encoder::new(&mut &mut pick_buf)).unwrap();

    loop {

        println!("Receiving...");

        let byte_msg = responder.recv_bytes(0).unwrap();

        println!("Received {:?}", byte_msg);

        let mut decoder = Decoder::new(&byte_msg[..]);
        let res: GameStatus = Decodable::decode(&mut decoder).ok().unwrap();
 
        println!("Received GameStatus {:?}", res);

        responder.send(&pick_buf, 0).unwrap();
        
        thread::sleep_ms(1000);

    }

}
