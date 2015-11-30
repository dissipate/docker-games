extern crate zmq;
use std::thread;

fn main() {
    let mut ctx = zmq::Context::new();

    let mut responder = ctx.socket(zmq::REP).unwrap();

    assert!(responder.bind("tcp://*:5555").is_ok());

    let mut msg = zmq::Message::new().unwrap();

    responder.set_rcvhwm(1).unwrap();

    loop {

        println!("Receiving...");

        responder.recv(&mut msg, 0).unwrap();

        println!("Received {}", msg.as_str().unwrap());
        
        responder.send_str("World", 0).unwrap();
        
        thread::sleep_ms(1000);

    }

}
