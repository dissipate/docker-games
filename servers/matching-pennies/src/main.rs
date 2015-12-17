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

#[derive(Debug)]
enum MatchingPenniesBot{
    MPB1,
    MPB2
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

fn flip_pick(pick: &Pick) -> Option<Pick>{

    match pick {
        &Pick { p: '1' } => Some( Pick { p: '0'} ),
        &Pick { p: '0' } => Some( Pick { p: '1'} ),
        _ => None
    }

}

fn round_win_select(picks: (&Pick, &Pick)) -> Option<MatchingPenniesBot>{
    
    match picks {
        (&Pick{ p: '0'}, &Pick{ p: '0'}) => Some(MatchingPenniesBot::MPB1),
        (&Pick{ p: '1'}, &Pick{ p: '1'}) => Some(MatchingPenniesBot::MPB1),
        (&Pick{ p: '0'}, &Pick{ p: '1'}) => Some(MatchingPenniesBot::MPB2),
        (&Pick{ p: '1'}, &Pick{ p: '0'}) => Some(MatchingPenniesBot::MPB2),
        _ => None
    }
}

fn check_for_winner(mpb1_wins: u8, mpb2_wins: u8, number_of_rounds: u8) -> (Option<MatchingPenniesBot>, f64){

    let expected_mpb1_wins = (number_of_rounds / 2) as f64;
    let expected_mpb2_wins = (number_of_rounds / 2) as f64;

    let mpb1_wins_f = mpb1_wins as f64;
    let mpb2_wins_f = mpb2_wins as f64;
    let number_of_rounds_f = number_of_rounds as f64;
    
    let win_factor =  (((mpb1_wins_f - expected_mpb1_wins).powf(2.0)) / number_of_rounds_f) + 
                      (((mpb2_wins_f - expected_mpb2_wins).powf(2.0)) / number_of_rounds_f);


    let mut winner = None;

    if (win_factor > 3.84) && (mpb1_wins > mpb2_wins)  { winner = Some(MatchingPenniesBot::MPB1); }
    else if (win_factor > 3.84) && (mpb2_wins > mpb1_wins) { winner = Some(MatchingPenniesBot::MPB2); }
    

    return (winner, win_factor);
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

    let mut mpb1_wins = 0;
    let mut mpb2_wins = 0;
    let mut total_rounds = 0;

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

        let winner = round_win_select((&mpb1_pick, &mpb2_pick)).unwrap();

        match winner {
            MatchingPenniesBot::MPB1 => mpb1_wins += 1,
            MatchingPenniesBot::MPB2 => mpb2_wins += 1
            
        }
 
        total_rounds += 1;

        let game_winner = check_for_winner(mpb1_wins, mpb2_wins, total_rounds); 

        println!("WINNER {:?}, RUNNING SCORES: {:?}, {:?}, {:?}", winner, mpb1_wins, mpb2_wins, game_winner);      

        println!("WINNER {:?}", winner); 

        println!("Received {:?}", mpb1_pick);
        println!("Received {:?}", mpb2_pick);

        mpb1_pick_buf.clear();
        mpb2_pick_buf.clear();

        mpb1_pick = flip_pick(&mpb1_pick).unwrap();

        mpb1_pick.encode(&mut Encoder::new(&mut &mut mpb1_pick_buf)).unwrap();
        mpb2_pick.encode(&mut Encoder::new(&mut &mut mpb2_pick_buf)).unwrap();
        
        std::thread::sleep(Duration::new(1,0));
    }

}
