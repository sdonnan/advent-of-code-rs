use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

struct Triangle {
    a : u32,
    b : u32,
    c : u32,
}

// Convert a string into a vector of directions
fn parse(text: &str) -> Result<Vec<Triangle>, String> {
    let mut vec: Vec<Triangle> = Vec::new();
    // for each line
    for (line, entry) in text.lines().enumerate() {
        let mut nums : [u32; 3] = [0,0,0];
        for (index, num_str) in entry.split_whitespace().enumerate() {
            match num_str.parse::<u32>() {
                Ok(n)  => {nums[index] = n;},
                Err(_) => {return Result::Err(format!("Bad input at line {}",line));}
            }
            vec.push(Triangle{a:nums[0],b:nums[1],c:nums[2]});
        }
    }
    Result::Ok(vec)
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
        let tris = parse(&s).unwrap();
        let mut count : usize = 0;
        for t in &tris {
           if t.a + t.b > t.c &&
              t.b + t.c > t.a &&
              t.c + t.a > t.b {
                  count += 1;
              }
        }
        println!("{} of {} triangles valid",count,tris.len());
    } else {
        process::exit(1);
    }
}
