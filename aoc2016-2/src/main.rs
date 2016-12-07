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
fn limit<T: Ord>(input: T, min: T, max: T) -> T {
    match () {
        _ if input < min => min,
        _ if input > max => max,
        _ => input
    }
}

const PT2_KEYS : [Option<i8>; 25] =
    [ None,    None,     Some(1),  None,     None,
      None,    Some(2),  Some(3),  Some(4),  None,
      Some(5), Some(6),  Some(7),  Some(8),  Some(9),
      None,    Some(10), Some(11), Some(12), None,
      None,    None,     Some(13), None,     None ];

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
        self.x = limit(key % 3 - 1,0,2);
        self.y = limit(key / 3    ,0,2);
    }

    // Keypad maps to coordinates like so
    //    0 1 2 3 4
    //    --------- x
    // 0 |    1
    // 1 |  2 3 4
    // 2 |5 6 7 8 9
    // 3 |  A B C
    // 4 |    D
    //   y
    // Return the integer key represented by the coordinate (A-D are hex)
    fn as_key_pt2(&self) -> Option<i8> { PT2_KEYS[(self.x+5*self.y) as usize] }

    // set the coordinate based on the keypad. see mapping in PT2_KEYS
    fn from_key_pt2(&mut self, key: i8) {
        let lin_pos = PT2_KEYS.iter().enumerate().find(
            |&x| match x.1 {
                &Some(y) => key == y,
                _       => false
            }
        ).unwrap().0;
        self.x = (lin_pos % 5) as i8;
        self.y = (lin_pos / 5) as i8;
    }

    // go to a coordinate limited to the keypad
    fn go(&mut self, dir: &Direction) {
        match dir {
            &Direction::Left  => {self.x = limit(self.x-1,0,2)},
            &Direction::Right => {self.x = limit(self.x+1,0,2)},
            &Direction::Up    => {self.y = limit(self.y-1,0,2)},
            &Direction::Down  => {self.y = limit(self.y+1,0,2)},
        }
    }

    // go to a coordinate limited to the part 2 keypad
    fn go_pt2(&mut self, dir: &Direction) {
        let new_coord = match dir {
            &Direction::Left  => Coordinate{x:limit(self.x-1,0,4), y:self.y},
            &Direction::Right => Coordinate{x:limit(self.x+1,0,4), y:self.y},
            &Direction::Up    => Coordinate{y:limit(self.y-1,0,4), x:self.x},
            &Direction::Down  => Coordinate{y:limit(self.y+1,0,4), x:self.x},
        };
        // if its a valid key then update coordinate
        if let Some(_) = new_coord.as_key_pt2() {
            *self = new_coord;
            //self.x = new_coord.x;
            //self.y = new_coord.y;
        }
    }

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
            for step in v { dest.go(step) };
            dest.as_key()
        }).collect();
        println!("{:?}", code);

        let mut dest2 = Coordinate {x:0,y:0};
        dest2.from_key_pt2(5); // instructions say we start at 5
        let code : Vec<String> = key_vecs.iter().map(|v| {
            for step in v { dest2.go_pt2(step) };
            format!("{:x}",dest2.as_key_pt2().unwrap()) // make it hex
        }).collect();
        println!("{:?}", code);
    } else {
        process::exit(1);
    }
}
