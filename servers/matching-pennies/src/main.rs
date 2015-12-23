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

#[derive(RustcEncodable, RustcDecodable, PartialEq, Debug)]
struct RoundStatus{
    //Round
    r: u64,
    //Pick
    p: char,
    //Score
    s: u64,
    //Status
    t: char
}

#[derive(Debug, PartialEq)]
enum MatchingPenniesBot{
    MPB1,
    MPB2,
    Neither
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

//fn flip_pick(pick: &Pick) -> Option<Pick>{

//    match pick {
//        &Pick { p: '1' } => Some( Pick { p: '0'} ),
//        &Pick { p: '0' } => Some( Pick { p: '1'} ),
//        _ => None
//    }

//}

fn round_win_select(picks: (&Pick, &Pick)) -> Option<MatchingPenniesBot>{
    
    match picks {
        (&Pick{ p: '0'}, &Pick{ p: '0'}) => Some(MatchingPenniesBot::MPB1),
        (&Pick{ p: '1'}, &Pick{ p: '1'}) => Some(MatchingPenniesBot::MPB1),
        (&Pick{ p: '0'}, &Pick{ p: '1'}) => Some(MatchingPenniesBot::MPB2),
        (&Pick{ p: '1'}, &Pick{ p: '0'}) => Some(MatchingPenniesBot::MPB2),
        _ => None
    }
}

fn check_for_winner(mpb1_wins: u64, mpb2_wins: u64, number_of_rounds: u64) -> (MatchingPenniesBot, f64){

    let expected_mpb1_wins = (number_of_rounds / 2) as f64;
    let expected_mpb2_wins = (number_of_rounds / 2) as f64;

    let mpb1_wins_f = mpb1_wins as f64;
    let mpb2_wins_f = mpb2_wins as f64;
    let number_of_rounds_f = number_of_rounds as f64;
    
    let win_factor =  (((mpb1_wins_f - expected_mpb1_wins).powf(2.0)) / number_of_rounds_f) + 
                      (((mpb2_wins_f - expected_mpb2_wins).powf(2.0)) / number_of_rounds_f);


    let mut winner = MatchingPenniesBot::Neither;

    if (win_factor > 3.84) && (mpb1_wins > mpb2_wins)  { winner = MatchingPenniesBot::MPB1; }
    else if (win_factor > 3.84) && (mpb2_wins > mpb1_wins) { winner = MatchingPenniesBot::MPB2; }
    

    return (winner, win_factor);
}

fn announce_winner_exit(mpb: MatchingPenniesBot) -> (){

    println!("Winner is {:?}!", mpb);
    std::process::exit(0);
}

fn get_round_statuses(round_number: u64, mpb1_pick: &Pick, mpb2_pick: &Pick, mpb1_wins: u64) -> (RoundStatus, RoundStatus){

    let mpb2_wins = round_number - mpb1_wins;

    let mpb1_pick_char = mpb1_pick.p;
    let mpb2_pick_char = mpb2_pick.p;

    let mpb1_pick_flip: char;

    let mpb1_game_status: char;
    let mpb2_game_status: char;

    let mpb1_round_status: RoundStatus;
    let mpb2_round_status: RoundStatus;
    
    match mpb1_pick_char {
        '0' => mpb1_pick_flip = '1',
        '1' => mpb1_pick_flip = '0',
        _ => mpb1_pick_flip = '0'
    }

    //return (RoundStatus{r: 9, p: '0', s: 99, t: 'c'}, RoundStatus{r: 9, p: '0', s: 99, t: 'c'});

    let (winner, score) = check_for_winner(mpb1_wins, mpb2_wins, round_number);

    let normalized_score = ((score / 3.84) * 100_000_000 as f64) as u64;

    //Round
    //r: u64,
    //Pick
    //p: char,
    //Score
    //s: u64,
    //Status
    //t: char

    if winner == MatchingPenniesBot::MPB1{
        mpb1_game_status = 'w';
        mpb2_game_status = 'l';
    }
    else if winner == MatchingPenniesBot::MPB2{
        mpb1_game_status = 'l';
        mpb2_game_status = 'w';
    }
    else if mpb1_wins > mpb2_wins{
        mpb1_game_status = 'e';
        mpb2_game_status = 'r';
    }
    else if mpb1_wins < mpb2_wins{
        mpb1_game_status = 'r';
        mpb2_game_status = 'e';
    }
    else {
        mpb1_game_status = 'n';
        mpb2_game_status = 'n';
    }

    mpb1_round_status = RoundStatus {r: round_number, p: mpb2_pick_char, s: normalized_score, t: mpb1_game_status};
    mpb2_round_status = RoundStatus {r: round_number, p: mpb1_pick_flip, s: normalized_score, t: mpb2_game_status};


    return (mpb1_round_status, mpb2_round_status);
    

}


fn main() {

    let mut context = zmq::Context::new();
    let mut mpb1 = context.socket(zmq::REQ).unwrap();
    let mut mpb2 = context.socket(zmq::REQ).unwrap();

    mpb1.set_rcvhwm(1).unwrap();
    mpb2.set_rcvhwm(1).unwrap();

    assert!(mpb1.connect("tcp://mpb1:5555").is_ok());

    assert!(mpb2.connect("tcp://mpb2:5555").is_ok());

    //let (mut mpb1_pick, mut mpb2_pick) = (Pick { p: '^' }, Pick { p: '^' });

    let (mut mpb1_status, mut mpb2_status) = (RoundStatus { r: 0, p: '^', s: 0, t: 'n' }, RoundStatus { r: 0, p: '^', s: 0, t: 'n' });

    let mut mpb1_status_buf = Vec::with_capacity(20);
    let mut mpb2_status_buf = Vec::with_capacity(20);

    let mut mpb1_wins = 0;
    let mut mpb2_wins = 0;
    let mut total_rounds = 0;

    let mut mpb1_pick: Pick;
    let mut mpb2_pick: Pick;

    mpb1_status.encode(&mut Encoder::new(&mut &mut mpb1_status_buf)).unwrap();
    mpb2_status.encode(&mut Encoder::new(&mut &mut mpb2_status_buf)).unwrap();

    loop {

        println!("Sending {:?}", mpb1_status_buf);
        println!("Sending {:?}", mpb2_status_buf);

        mpb1.send(&mpb2_status_buf, 0).unwrap();
        mpb2.send(&mpb1_status_buf, 0).unwrap();

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
            MatchingPenniesBot::MPB2 => mpb2_wins += 1,
            _ => ()    
        }
 
        total_rounds += 1;

        let (game_winner_opt, factor) = check_for_winner(mpb1_wins, mpb2_wins, total_rounds);

        let game_winner:MatchingPenniesBot;

        match game_winner_opt {
        MatchingPenniesBot::MPB1 => game_winner = MatchingPenniesBot::MPB1,
        MatchingPenniesBot::MPB2 => game_winner = MatchingPenniesBot::MPB2,
        _ => game_winner = MatchingPenniesBot::Neither 
        }  
 
        println!("WINNER {:?}, RUNNING SCORES: {:?}, {:?}, {:?}, {:?}", winner, mpb1_wins, mpb2_wins, game_winner, factor);      

        println!("WINNER {:?}", winner);

        match game_winner {
            MatchingPenniesBot::MPB1 | MatchingPenniesBot::MPB2 => announce_winner_exit(game_winner),
            _ => () 
           
        }

        println!("Received {:?}", mpb1_pick);
        println!("Received {:?}", mpb2_pick);

        mpb1_status_buf.clear();
        mpb2_status_buf.clear();
       
        let new_statuses = get_round_statuses(total_rounds, &mpb1_pick, &mpb2_pick, mpb1_wins);

        mpb1_status = new_statuses.0;
        mpb2_status = new_statuses.1;

        //mpb1_pick = flip_pick(&mpb1_pick).unwrap();

        mpb1_status.encode(&mut Encoder::new(&mut &mut mpb1_status_buf)).unwrap();
        mpb2_status.encode(&mut Encoder::new(&mut &mut mpb2_status_buf)).unwrap();
        
        std::thread::sleep(Duration::new(1,0));
    }

}
