extern crate clap;
use clap::{Arg, App};

pub mod set1;
pub mod storage;
pub mod challenge;

fn main() {
    let matches = App::new("Cryptopals")
        .version("0.1.0")
        .author("Lucas Reynolds <lreynolds18@gmail.com>")
        .about("My 48 exercises that demonstrate attacks on real-world crypto in Rust")
        .arg(Arg::with_name("set")
                 .short("s")
                 .long("set")
                 .takes_value(true)
                 .help("Which set should run"))
        .arg(Arg::with_name("challenge")
                 .short("c")
                 .long("challenge")
                 .takes_value(true)
                 .help("Which challenge should run"))
        .get_matches();

    let set = matches
      .value_of("set").expect("Please include a challenge number.")
      .parse::<i32>().expect("Please make sure challenge is a number!");

    let challenge = matches
      .value_of("challenge").expect("Please include a challenge number.")
      .parse::<i32>().expect("Please make sure challenge is a number!");

    match set {
        1 => match challenge {
            1 => set1::challenge1(),
            2 => set1::challenge2(),
            3 => set1::challenge3(),
            4 => set1::challenge4(),
            5 => set1::challenge5(),
            6 => set1::challenge6(),
            7 => set1::challenge7(),
            _ => println!("Please include a valid challenge! {}", challenge)
        },
        _ => println!("Please include a valid set! {}", set)
    }
}
