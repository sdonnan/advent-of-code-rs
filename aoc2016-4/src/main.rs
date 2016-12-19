use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::collections::HashMap;

extern crate regex;
use regex::Regex;

#[derive(Debug)]
struct Room {
    id : String,
    check : String,
    sector : usize,
}

impl Room {

    fn is_real(&self) -> bool {
        let mut letter_count : HashMap<char,usize> = HashMap::new();
        for c in self.id.chars() {
            let count = letter_count.entry(c).or_insert(0);
            *count += 1;
        }
        let mut order = letter_count.iter().collect::<Vec<_>>();
        // since sorting is stable this order of operations guarantees the output is sorted by
        // number of occurances and then alphabetically
        order.sort_by_key(|entry| entry.0); // sort alphabetically
        order.sort_by(|a,b| b.1.cmp(a.1)); // sort by count
        let real_sum = order[0..5].iter().map(|e| e.0).cloned().collect::<String>();
        return real_sum == self.check;
    }
}

// Convert a string into a vector of directions
fn parse(text: &str) -> Result<Vec<Room>, String> {
    let mut rooms: Vec<Room> = Vec::new();
    let re = Regex::new(r"^([0-9]*)\[([a-zA-Z]{5})\]$").unwrap();
    // for each line
    for entry in text.lines() {
        // split on dashes
        let mut chunks = entry.split('-');
        // reverse iterate to get sector, checksum
        let tail = re.captures(chunks.next_back().unwrap()).unwrap();
        let sector = tail.at(1).unwrap().parse::<usize>().unwrap();
        let checksum = tail.at(2).unwrap();
        // get room letters
        let room : String = chunks.collect::<String>();

        rooms.push(
            Room {
                id : room,
                check : checksum.to_string(),
                sector : sector
            }
        );
    }
    Result::Ok(rooms)
}

fn sum_real_rooms(rooms: &Vec<Room>) -> usize {
    return rooms.iter()
                .filter(|x| x.is_real())
                .map(|room| room.sector)
                .sum();
}

const BASE : u32 = 'a' as u32;
const MOD  : u32 = 'z' as u32 - BASE + 1;

fn shift(c: char, count: u32) -> char {
    ((((c as u32) - BASE + count) % MOD) + BASE) as u8 as char
}

fn main() {
    // get the file contents as an Option
    let option = match env::args().count() {
        // correct number of args? try to read the file
        2 => {
            let fname = env::args().nth(1).unwrap();
            match File::open(&fname) {
                // create string with file contents
                Ok(mut x) => {
                    let mut s = String::new();
                    if let Ok(_) = x.read_to_string(&mut s) {
                        Option::Some(s)
                    }
                    else {
                        println!("Error reading file '{}'",&fname);
                        Option::None
                    }
                }
                Err(e) => {
                    println!("Couldn't open file '{}': {}",&fname,e);
                    Option::None
                }
            }
        }
        // otherwise print usage
        _ => {
            let name = env::args().nth(0).unwrap();
            println!("Usage: {} input-filename", &name);
            Option::None
        }
    };

    // if there is a string, parse it
    if let Some(s) = option {
        let rooms = parse(&s).unwrap();
        println!("Sum of valid room sectors: {}",sum_real_rooms(&rooms));
        println!("Rooms with 'north' in the name:");
        for room in rooms.iter().filter(|r| r.is_real()) {
            let decrypted = room.id.chars()
                                   .map(|c| shift(c,room.sector as u32))
                                   .collect::<String>();
            if let Some(_) = decrypted.find("north") {
                println!("Name: {}; {:?}",decrypted,room);
            }
        }
    } else {
        process::exit(1);
    }
}

#[test]
fn test_valid() {
    let test_input =
        "aaaaa-bbb-z-y-x-123[abxyz]\n\
         a-b-c-d-e-f-g-h-987[abcde]\n\
         not-a-real-room-404[oarel]\n\
         totally-real-room-200[decoy]";
    let rooms = parse(&test_input).unwrap();
    assert_eq!(sum_real_rooms(&rooms), 1514);
}

#[test]
fn test_shift() {
    assert_eq!(shift('z',1),'a');
    assert_eq!(shift('a',1),'b');
}
