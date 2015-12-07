extern crate msgpack;

extern crate rustc_serialize;

#[derive(RustcEncodable, RustcDecodable)]
struct MyStruct {
  p: u8,
  c: String
}

fn main() {

  let some_struct: MyStruct = MyStruct {p: 1, c: "blah".to_string()};

  let str = msgpack::Encoder::to_msgpack(&some_struct).ok().unwrap();
  println!("Encoded: {:?}", str);

  let dec: Vec<String> = msgpack::from_msgpack(&str).ok().unwrap();
  println!("Decoded: {:?}", dec);
}
