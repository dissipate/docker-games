extern crate zmq;
use std::thread;

fn main() {
    let mut ctx = zmq::Context::new();

    let mut responder = ctx.socket(zmq::REP).unwrap();

    assert!(responder.bind("tcp://*:5555").is_ok());

    //let mut msg: Vec<u8> = Vec::new();

    responder.set_rcvhwm(1).unwrap();

    loop {

        println!("Receiving...");

        let byte_msg = responder.recv_bytes(0).unwrap();

        println!("Received {}", byte_msg.get(0).unwrap());
        
        responder.send_str("World", 0).unwrap();
        
        thread::sleep_ms(1000);

    }

}
