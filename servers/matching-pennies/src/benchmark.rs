extern crate zmq;
use std::thread;

fn main() {


    let mut context = zmq::Context::new();
    let mut requester = context.socket(zmq::REQ).unwrap();

    requester.set_rcvhwm(1).unwrap();

    assert!(requester.connect("tcp://mpb:5555").is_ok());

    let mut msg_req = zmq::Message::new().unwrap();

    for _ in 0..1_000_000 {

        //println!("Sending Hello");
        requester.send(b"Hello", 0).unwrap();

        requester.recv(&mut msg_req, 0).unwrap();
        //println!("Received {}", msg_req.as_str().unwrap());

        //thread::sleep_ms(1000);
    }

}
