extern crate zmq;
use std::thread;

fn main() {
    let mut ctx = zmq::Context::new();

    let mut responder = ctx.socket(zmq::REP).unwrap();

    assert!(responder.bind("tcp://*:5555").is_ok());

    let mut msg = zmq::Message::new().unwrap();

    responder.set_rcvhwm(1).unwrap();

    println!("Starting...");

    loop {

        responder.recv(&mut msg, 0).unwrap();
        
        responder.send_str("World", 0).unwrap();

    }

}
