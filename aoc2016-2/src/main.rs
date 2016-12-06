use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

#[derive(Debug, Clone, Copy)]
enum Direction { Left, Right, Up, Down }

// Convert a string into a vector of directions
fn parse(text: &str) -> Result<Vec<Vec<Direction>>, String> {
    let mut vec: Vec<Vec<Direction>> = Vec::new();
    // for each line
    for entry in text.split_whitespace() {
        // create a vector of steps based on the characters
        let mut key_vec: Vec<Direction> = Vec::new();
        let chars = entry.chars();
        for c in chars {
            let dir = match c {
                'L' => Direction::Left,
                'R' => Direction::Right,
                'D' => Direction::Down,
                'U' => Direction::Up,
                _   => {return Result::Err(format!("Invalid direction at '{}'",entry))}
            };
            key_vec.push(dir);
        }
        vec.push(key_vec);
    }
    Result::Ok(vec)
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Coordinate {
    x : i8,
    y : i8
}

// limit a number to the keypad coordinates
fn key_lim(input: i8) -> i8 {
    match () {
        _ if input < 0 => 0,
        _ if input > 2 => 2,
        _ => input
    }
}

impl Coordinate {

    // Keypad maps to coordinates like so
    //    0 1 2
    //    ----- x
    // 0 |1 2 3
    // 1 |4 5 6
    // 2 |7 8 9
    //   y
    // Return the integer key represented by the coordinate
    fn as_key(&self) -> i8 { 1+self.x+3*self.y }

    // set the coordinate based on the keypad. see mapping in as_key
    fn from_key(&mut self, key: i8) {
        self.x = key_lim(key % 3 - 1);
        self.y = key_lim(key / 3);
    }

    // go to a coordinate limited to the keypad
    fn go(&mut self, dir: &Direction) {
        match dir {
            &Direction::Left  => {self.x = key_lim(self.x-1)},
            &Direction::Right => {self.x = key_lim(self.x+1)},
            &Direction::Up    => {self.y = key_lim(self.y-1)},
            &Direction::Down  => {self.y = key_lim(self.y+1)},
        }
    }
}

// calculate the final destination coordinates
fn calc_dest(start: &Coordinate, steps: &Vec<Direction>) -> Coordinate {
    let mut location : Coordinate = start.clone();
    for step in steps {
        location.go(step);
    };
    location
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
        let key_vecs = parse(&s).unwrap();
        let mut dest = Coordinate {x:0,y:0};
        dest.from_key(5); // instructions say we start at 5
        let code : Vec<i8> = key_vecs.iter().map(|v| {
            dest = calc_dest(&dest, &v);
            dest.as_key()
        }).collect();
        println!("{:?}", code);
    } else {
        process::exit(1);
    }
}
