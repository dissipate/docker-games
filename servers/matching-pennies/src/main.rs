extern crate zmq;
use std::thread::sleep;
use std::time::Duration;

extern crate rmp_serialize as msgpack;
extern crate rustc_serialize;

use rustc_serialize::{Encodable, Decodable};
use msgpack::{Encoder, Decoder};

#[derive(RustcEncodable, RustcDecodable, PartialEq, Debug, Clone, Copy)]
struct Pick{
    p: char
}

#[derive(RustcEncodable, RustcDecodable, PartialEq, Debug, Clone, Copy)]
struct GameStatus{
    //Round
    r: u64,
    //Pick
    p: char,
    //Score
    s: u64,
    //Status
    t: char 
}

#[derive(RustcEncodable, RustcDecodable, PartialEq, Debug, Clone, Copy)]
struct GameStatusInternal{
    //Round
    round: u64,
    //Pick
    pick: PickVal,
    //Score
    score: u64,
    //Status
    status: GameStatusVal 
}

#[derive(RustcEncodable, RustcDecodable, PartialEq, Debug, Clone, Copy)]
enum MatchingPenniesBot{
    MPB1,
    MPB2,
    Neither
}

#[derive(RustcEncodable, RustcDecodable, PartialEq, Debug, Clone, Copy)]
enum GameStatusVal{
    WinScore,
    LossScore,
    WinTime,
    LossTime,
    WinForfeit,
    LossForfeit,
    Lead,
    Trail,
    Neutral
} 

#[derive(RustcEncodable, RustcDecodable, PartialEq, Eq, Debug, Clone, Copy)]
enum PickVal{
    Zero,
    One,
    Failure
}

#[derive(RustcEncodable, RustcDecodable, PartialEq, Eq, Debug, Clone, Copy)]
enum PickStatus{
    Valid,
    ParseError,
    TimeOut
}

#[derive(RustcEncodable, RustcDecodable, PartialEq, Debug, Clone, Copy)]
struct PickInternal{
    pick: PickVal,
    status: PickStatus
}

// fn parse_pick(pick: &[u8]) -> Option<u8>{

//     let mut decoder = Decoder::new(&pick[..]);

//     let res = Decodable::decode(&mut decoder).ok();

//     match res {
//         Some(Pick { p: '0' }) => Some(0),
//         Some(Pick { p: '1' }) => Some(1),
//         _ => None
//     }     
    
// }

//fn flip_pick(pick: &Pick) -> Option<Pick>{

//    match pick {
//        &Pick { p: '1' } => Some( Pick { p: '0'} ),
//        &Pick { p: '0' } => Some( Pick { p: '1'} ),
//        _ => None
//    }

//}

// fn round_win_select(picks: (&PickInternal, &PickInternal)) -> MatchingPenniesBot{

//     let pick_vals = (picks.0.pick, picks.1.pick)
    
//     match pick_vals {
//         (PickVal::Zero, PickVal::Zero) => MatchingPenniesBot::MPB1,
//         (PickVal::One, PickVal::One) => MatchingPenniesBot::MPB1,
//         (PickVal::Zero, PickVal::One) => MatchingPenniesBot::MPB2,
//         (PickVal::One, PickVal::Zero) => MatchingPenniesBot::MPB2,
//         (PickVal::Zero, PickVal::Failure) => MatchingPenniesBot::MPB1,
//         (PickVal::One, PickVal::Failure) => MatchingPenniesBot::MPB1,
//         (PickVal::Failure, PickVal::Zero) => MatchingPenniesBot::MPB2,
//         (PickVal::Failure, PickVal::One) => MatchingPenniesBot::MPB2,
//         (PickVal::Failure, PickVal::Failure) => MatchingPenniesBot::Neither
//     }
// }

fn calculate_score(mpb1_wins: u64, mpb2_wins: u64, number_of_rounds: u64) -> u64{

    let expected_mpb1_wins = (number_of_rounds / 2) as f64;
    let expected_mpb2_wins = (number_of_rounds / 2) as f64;

    let mpb1_wins_f = mpb1_wins as f64;
    let mpb2_wins_f = mpb2_wins as f64;

    let number_of_rounds_f = number_of_rounds as f64;
    
    let win_factor =  (((mpb1_wins_f - expected_mpb1_wins).powf(2.0)) / number_of_rounds_f) + 
                      (((mpb2_wins_f - expected_mpb2_wins).powf(2.0)) / number_of_rounds_f);

    println!("mpb1_wins: {:?}, mpb2_wins: {:?}, number of rounds: {:?}", mpb1_wins, mpb2_wins, number_of_rounds);

    let score = (((win_factor / 3.84) * 100_000_000 as f64).trunc()) as u64;

    return score;
}

fn check_for_score_winner(mpb1_wins: u64, mpb2_wins: u64, number_of_rounds: u64) -> 
   (MatchingPenniesBot, u64){   

    let score = calculate_score(mpb1_wins, mpb2_wins, number_of_rounds);

    let mut winner = MatchingPenniesBot::Neither;

    if (score >= 100_000_000) && (mpb1_wins > mpb2_wins)  { winner = MatchingPenniesBot::MPB1; }
    else if (score >= 100_000_000) && (mpb2_wins > mpb1_wins) { winner = MatchingPenniesBot::MPB2; }
    

    return (winner, score);
}

fn announce_winner_exit(mpb: MatchingPenniesBot) -> (){

    println!("Winner is {:?}!", mpb);
    std::process::exit(0);

}

fn get_game_statuses(round_number: u64, mpb1_pick: &PickInternal, mpb2_pick: &PickInternal, mpb1_wins: u64) -> (GameStatusInternal, GameStatusInternal, u64){

    let mut mpb2_wins_current = (round_number - 1) - mpb1_wins;
    let mut mpb1_wins_current = mpb1_wins;

    let mpb1_pick_flip: PickVal;

    let mpb1_pick_val = mpb1_pick.pick.clone();
    let mpb2_pick_val = mpb2_pick.pick.clone();

    let mpb1_pick_status_val = mpb1_pick.status.clone();
    let mpb2_pick_status_val = mpb2_pick.status.clone();

    let mut mpb1_game_status_val = GameStatusVal::Neutral;
    let mut mpb2_game_status_val = GameStatusVal::Neutral;

    let mpb1_game_status: GameStatusInternal;
    let mpb2_game_status: GameStatusInternal;

    let mut picks_were_valid = true;
    
    match mpb1_pick_val {
        PickVal::Zero => mpb1_pick_flip = PickVal::One,
        PickVal::One => mpb1_pick_flip = PickVal::Zero,
        _ => mpb1_pick_flip = PickVal::Failure
    }

    if(mpb1_pick_val == PickVal::Failure) || (mpb2_pick_val == PickVal::Failure){

        let mpb_pick_status_tuple = (mpb1_pick_status_val, mpb2_pick_status_val);

        let mut game_status_tuple = (GameStatusVal::Neutral, GameStatusVal::Neutral);

        match mpb_pick_status_tuple {
            (PickStatus::Valid, PickStatus::Valid) => (),
            (PickStatus::Valid, PickStatus::ParseError) => game_status_tuple = (GameStatusVal::WinForfeit, GameStatusVal::LossForfeit),
            (PickStatus::Valid, PickStatus::TimeOut) => game_status_tuple = (GameStatusVal::WinTime, GameStatusVal::LossTime),
            (PickStatus::ParseError, PickStatus::Valid) => game_status_tuple = (GameStatusVal::LossForfeit, GameStatusVal::WinForfeit),
            (PickStatus::ParseError, PickStatus::ParseError) => game_status_tuple = (GameStatusVal::LossForfeit, GameStatusVal::LossForfeit),
            (PickStatus::ParseError, PickStatus::TimeOut) => game_status_tuple = (GameStatusVal::LossForfeit, GameStatusVal::LossTime),
            (PickStatus::TimeOut, PickStatus::Valid) => game_status_tuple = (GameStatusVal::LossTime, GameStatusVal::WinTime),
            (PickStatus::TimeOut, PickStatus::ParseError) => game_status_tuple = (GameStatusVal::LossTime, GameStatusVal::LossForfeit),
            (PickStatus::TimeOut, PickStatus::TimeOut) => game_status_tuple = (GameStatusVal::LossTime, GameStatusVal::LossTime)
        }

        mpb1_game_status_val = game_status_tuple.0;
        mpb2_game_status_val = game_status_tuple.1;

        picks_were_valid = false;

    }
    else {

        let mpb_pick_tuple = (mpb1_pick_val, mpb2_pick_val);

        match mpb_pick_tuple {
            (PickVal::Zero, PickVal::Zero) => mpb1_wins_current += 1,
            (PickVal::One, PickVal::One) => mpb1_wins_current += 1,
            (PickVal::Zero, PickVal::One) => mpb2_wins_current += 1,
            (PickVal::One, PickVal::Zero) => mpb2_wins_current += 1,
            (_,_) => ()
        }
        
    }

    let winner_score = check_for_score_winner(mpb1_wins_current, mpb2_wins_current, round_number);

    let winner = winner_score.0;
    let score = winner_score.1;

    if picks_were_valid {
         if winner == MatchingPenniesBot::MPB1{
            mpb1_game_status_val = GameStatusVal::WinScore;
            mpb2_game_status_val = GameStatusVal::LossScore;
        }
        else if winner == MatchingPenniesBot::MPB2 {
            mpb1_game_status_val = GameStatusVal::LossScore;
            mpb2_game_status_val = GameStatusVal::WinScore;
        }
        else if mpb1_wins_current > mpb2_wins_current {
            mpb1_game_status_val = GameStatusVal::Lead;
            mpb2_game_status_val = GameStatusVal::Trail;
        }
        else if mpb1_wins_current < mpb2_wins_current {
            mpb1_game_status_val = GameStatusVal::Trail;
            mpb2_game_status_val = GameStatusVal::Lead;
        }
        else {
            mpb1_game_status_val = GameStatusVal::Neutral;
            mpb2_game_status_val = GameStatusVal::Neutral;
        }
    }

    mpb1_game_status = GameStatusInternal {round: round_number, pick: mpb2_pick_val, score: score, status: mpb1_game_status_val};
    mpb2_game_status = GameStatusInternal {round: round_number, pick: mpb1_pick_flip, score: score, status: mpb2_game_status_val};


    return (mpb1_game_status, mpb2_game_status, mpb1_wins_current);
}

fn get_pick_internal(pick_option: &Option<Pick>) -> PickInternal {

    match pick_option {
        &Some(Pick{ p: '0' }) => PickInternal{pick: PickVal::Zero, status: PickStatus::Valid},
        &Some(Pick{ p: '1' }) => PickInternal{pick: PickVal::One, status: PickStatus::Valid},
        _ => PickInternal{pick: PickVal::Failure, status: PickStatus::ParseError}
    }
}

fn gsi_to_gs(mpb_game_status: &GameStatusInternal) -> GameStatus{

    let round = mpb_game_status.round;
    let pick = mpb_game_status.pick.clone();
    let score = mpb_game_status.score;
    let status = mpb_game_status.status.clone();

    let status_char:char;
    let pick_char:char;

    match pick {
        PickVal::Zero => pick_char = '0',
        PickVal::One => pick_char = '1',
        PickVal::Failure => pick_char = '~'
    }

    match status {
        GameStatusVal::WinScore => status_char = 'w',
        GameStatusVal::LossScore => status_char = 'l',
        GameStatusVal::WinTime => status_char = 't',
        GameStatusVal::LossTime => status_char = 'i',
        GameStatusVal::WinForfeit => status_char = 'f',
        GameStatusVal::LossForfeit => status_char = 'o',
        GameStatusVal::Lead => status_char = 'e',
        GameStatusVal::Trail => status_char = 'r',
        GameStatusVal::Neutral => status_char = 'n'
    }

    return GameStatus{r: round, p: pick_char, s: score, t: status_char};
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

    let (mut mpb1_status, mut mpb2_status) = (GameStatus{r: 0, p: '^', s: 0, t: 'n'}, GameStatus{r: 0, p: '^', s: 0, t: 'n'});

    let mut mpb1_status_buf = Vec::with_capacity(20);
    let mut mpb2_status_buf = Vec::with_capacity(20);

    let mut mpb1_wins = 0;
    let mut total_rounds = 0;

    let mut mpb1_pick: Option<Pick>;
    let mut mpb2_pick: Option<Pick>;

    let mut mpb1_pick_internal: PickInternal;
    let mut mpb2_pick_internal: PickInternal;

    mpb1_status.encode(&mut Encoder::new(&mut &mut mpb1_status_buf)).unwrap();
    mpb2_status.encode(&mut Encoder::new(&mut &mut mpb2_status_buf)).unwrap();

    loop {

        println!("Sending {:?}", mpb1_status_buf);
        println!("Sending {:?}", mpb2_status_buf);

        mpb1.send(&mpb2_status_buf, 0).unwrap();
        mpb2.send(&mpb1_status_buf, 0).unwrap();

        let mpb1_byte_msg = mpb1.recv_bytes(0).unwrap();
        let mpb2_byte_msg = mpb2.recv_bytes(0).unwrap();       

        println!("Received {:?}", mpb1_byte_msg);
        println!("Received {:?}", mpb2_byte_msg);

        let mut decoder1 = Decoder::new(&mpb1_byte_msg[..]);
        let mut decoder2 = Decoder::new(&mpb2_byte_msg[..]);

        mpb1_pick = Decodable::decode(&mut decoder1).ok();
        mpb2_pick = Decodable::decode(&mut decoder2).ok();

        mpb1_pick_internal = get_pick_internal(&mpb1_pick);
        mpb2_pick_internal = get_pick_internal(&mpb2_pick);
 
        total_rounds += 1;

        println!("Received {:?}", mpb1_pick_internal);
        println!("Received {:?}", mpb2_pick_internal);

        mpb1_status_buf.clear();
        mpb2_status_buf.clear();
       
        let new_statuses = get_game_statuses(total_rounds, &mpb1_pick_internal, &mpb2_pick_internal, mpb1_wins);

        mpb1_wins = new_statuses.2;

        mpb1_status = gsi_to_gs(&new_statuses.0);
        mpb2_status = gsi_to_gs(&new_statuses.1);

        //mpb1_pick = flip_pick(&mpb1_pick).unwrap();

        mpb1_status.encode(&mut Encoder::new(&mut &mut mpb1_status_buf)).unwrap();
        mpb2_status.encode(&mut Encoder::new(&mut &mut mpb2_status_buf)).unwrap();
        
        std::thread::sleep(Duration::new(1,0));
    }

}
