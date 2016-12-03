use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

#[derive(Debug, Clone, Copy)]
enum Direction { Left, Right }

#[derive(Debug, Clone, Copy)]
struct Step {
    dir  : Direction,
    dist : isize
}

fn parse(text: &str) -> Result<Vec<Step>, String> {
    let mut vec: Vec<Step> = Vec::new();
    for entry in text.split(',') {
        let e: &str = entry.trim();
        if e.len() <= 1 {
            return Result::Err(format!("Invalid input at '{}'",entry));
        }
        let mut chars = e.chars();
        let dir = match chars.next().unwrap() {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _   => {return Result::Err(format!("Invalid direction at '{}'",entry))}
        };
        let dist = if let Ok(i) = e.split_at(1).1.parse::<isize>() {
            i
        } else {
            return Result::Err(format!("Invalid distance at '{}'",entry));
        };
        vec.push(Step {dir: dir,dist: dist});
    }
    Result::Ok(vec)
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Coordinate {
    x : isize,
    y : isize
}

enum AbsDir { North, East, South, West }

fn turn(a: AbsDir, t: Direction) -> AbsDir {
    match t {
        Direction::Left => match a {
            AbsDir::North => AbsDir::West,
            AbsDir::West  => AbsDir::South,
            AbsDir::South => AbsDir::East,
            AbsDir::East  => AbsDir::North
        },
        Direction::Right => match a {
            AbsDir::North => AbsDir::East,
            AbsDir::East  => AbsDir::South,
            AbsDir::South => AbsDir::West,
            AbsDir::West  => AbsDir::North,
        }
    }
}

// assume the following releationships
// -x w
// x  e
// -y s
// y  n
fn calc_dest(steps: &Vec<Step>) -> Coordinate {
    let mut facing = AbsDir::North;
    let mut location = Coordinate {x:0,y:0};
    for step in steps {
        facing = turn(facing, step.dir);
        match facing {
            AbsDir::North => location.y += step.dist,
            AbsDir::South => location.y -= step.dist,
            AbsDir::East  => location.x += step.dist,
            AbsDir::West  => location.x -= step.dist,
        }
    }
    location
}

fn calc_dest2(steps: &Vec<Step>) -> Coordinate {
    let mut facing = AbsDir::North;
    let mut location = Coordinate {x:0,y:0};
    let mut path : Vec<Coordinate> = Vec::new();
    path.reserve(steps.len() + 1);
    path.push(location);
    for step in steps {
        facing = turn(facing, step.dir);
        for _ in 0..step.dist {
            match facing {
                AbsDir::North => location.y += 1,
                AbsDir::South => location.y -= 1,
                AbsDir::East  => location.x += 1,
                AbsDir::West  => location.x -= 1,
            }
            // leave early if we already visited this spot
            if let Some(_) = path.iter().position(|c| c.x == location.x && c.y == location.y ) {return location};
            path.push(location);
        }
    }
    location
}

fn main() {
    // get the file contents
    let option = match env::args().count() {
        // correct number of args? try to read the file
        2 => {
            let fname = env::args().nth(1).unwrap();
            match File::open(&fname) {
                // create string with file contents
                Ok(mut x) => {
                    let mut s = String::new();
                    if let Ok(_) = x.read_to_string(&mut s) { Option::Some(s) }
                    else { println!("Error reading file '{}'",&fname); Option::None }
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

    if let Some(s) = option {
        let v = parse(&s).unwrap();
        let dest = calc_dest(&v);
        println!("Destination: {:?}. Distance: {}",dest, dest.x.abs()+dest.y.abs());
        let pt2_dest = calc_dest2(&v);
        println!("Destination: {:?}. Distance: {}",pt2_dest, pt2_dest.x.abs()+pt2_dest.y.abs());
    } else {
        process::exit(1);
    }
}
