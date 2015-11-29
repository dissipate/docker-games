extern crate zmq;
use std::thread;

fn main() {
    let mut ctx = zmq::Context::new();

    let mut responder = ctx.socket(zmq::REP).unwrap();

    assert!(responder.bind("tcp://*:5555").is_ok());

    let mut msg = zmq::Message::new().unwrap();

    let mut context = zmq::Context::new();
    let mut requester = context.socket(zmq::REQ).unwrap();

    assert!(requester.connect("tcp://localhost:5555").is_ok());

    let mut msg_req = zmq::Message::new().unwrap();

    loop {

        println!("Sending Hello");
        requester.send(b"Hello", 0).unwrap();
        
        responder.recv(&mut msg, 0).unwrap();

        println!("Received {}", msg.as_str().unwrap());
        responder.send_str("World", 0).unwrap();
        thread::sleep_ms(1000);

        requester.recv(&mut msg_req, 0).unwrap();
        println!("Received World {}", msg_req.as_str().unwrap());
    }

}
