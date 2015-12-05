extern crate zmq;
use std::thread;

fn main() {


    let mut context = zmq::Context::new();
    let mut mpb1 = context.socket(zmq::REQ).unwrap();
    let mut mpb2 = context.socket(zmq::REQ).unwrap();

    mpb1.set_rcvhwm(1).unwrap();
    mpb2.set_rcvhwm(1).unwrap();

    assert!(mpb1.connect("tcp://mpb1:5555").is_ok());

    assert!(mpb2.connect("tcp://mpb2:5555").is_ok());

    let mut msg_req1 = zmq::Message::new().unwrap();
    let mut msg_req2 = zmq::Message::new().unwrap();

    loop {

        println!("Sending Hello1");
        mpb1.send(b"Hello", 0).unwrap();

        mpb1.recv(&mut msg_req1, 0).unwrap();
        println!("Received {}", msg_req1.as_str().unwrap());

        println!("Sending Hello2");
        mpb2.send(b"Hello", 0).unwrap();

        mpb2.recv(&mut msg_req2, 0).unwrap();
        println!("Received {}", msg_req2.as_str().unwrap());


        thread::sleep_ms(1000);
    }

}
