extern crate zmq;
use std::thread::sleep;
use std::time::Duration;

extern crate rmp_serialize as msgpack;
extern crate rustc_serialize;

use rustc_serialize::{Encodable, Decodable};
use msgpack::{Encoder, Decoder};

#[derive(RustcEncodable, RustcDecodable, PartialEq, Debug)]
struct Pick{
    p: char
}

fn parse_pick(pick: &[u8]) -> Option<u8>{

    let mut decoder = Decoder::new(&pick[..]);

    let res = Decodable::decode(&mut decoder).ok();

    match res {
        Some(Pick { p: '0' }) => Some(0),
        Some(Pick { p: '1' }) => Some(1),
        _ => None
    }     
    
}


fn main() {

    let mut context = zmq::Context::new();
    let mut mpb1 = context.socket(zmq::REQ).unwrap();
    let mut mpb2 = context.socket(zmq::REQ).unwrap();

    mpb1.set_rcvhwm(1).unwrap();
    mpb2.set_rcvhwm(1).unwrap();

    assert!(mpb1.connect("tcp://mpb1:5555").is_ok());

    assert!(mpb2.connect("tcp://mpb2:5555").is_ok());

    let (mut mpb1_pick, mut mpb2_pick) = (Pick { p: '^' }, Pick { p: '^' });

    let mut mpb1_pick_buf = Vec::with_capacity(20);
    let mut mpb2_pick_buf = Vec::with_capacity(20);

    mpb1_pick.encode(&mut Encoder::new(&mut &mut mpb1_pick_buf)).unwrap();
    mpb2_pick.encode(&mut Encoder::new(&mut &mut mpb2_pick_buf)).unwrap();

    loop {

        println!("Sending {:?}", mpb1_pick_buf);
        println!("Sending {:?}", mpb2_pick_buf);

        mpb1.send(&mpb2_pick_buf, 0).unwrap();
        mpb2.send(&mpb1_pick_buf, 0).unwrap();

        let mpb1_byte_msg = mpb1.recv_bytes(0).unwrap();
        let mpb2_byte_msg = mpb2.recv_bytes(0).unwrap();

        let parsed_pick = parse_pick(&mpb1_byte_msg);        

        println!("Received {:?}", mpb1_byte_msg);
        println!("Received {:?}", mpb2_byte_msg);
        println!("Received {:?}", parsed_pick);

        let mut decoder1 = Decoder::new(&mpb1_byte_msg[..]);
        let mut decoder2 = Decoder::new(&mpb2_byte_msg[..]);

        mpb1_pick = Decodable::decode(&mut decoder1).ok().unwrap();
        mpb2_pick = Decodable::decode(&mut decoder2).ok().unwrap();

        println!("Received {:?}", mpb1_pick);
        println!("Received {:?}", mpb2_pick);

        mpb1_pick_buf.clear();
        mpb2_pick_buf.clear();

        mpb1_pick.encode(&mut Encoder::new(&mut &mut mpb1_pick_buf)).unwrap();
        mpb2_pick.encode(&mut Encoder::new(&mut &mut mpb2_pick_buf)).unwrap();
        
        std::thread::sleep(Duration::new(1,0));
    }

}
