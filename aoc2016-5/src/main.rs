use std::io;
use std::ops::Range;

extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;

#[derive(Debug)]
struct Pwchar {
    chr : char,
    pos : usize
}

fn calc_password_chunk<T: Digest>(hasher: &mut T,
                                  input : &str,
                                  range: Range<usize>) -> Vec<(char, char)> {
    let mut result : Vec<(char,char)> = Vec::new();
    let mut hash = vec![ 0u8; hasher.output_bytes() ];
    for index in range {
        let teststr = format!("{}{:01}",input,index);
        hasher.reset();
        hasher.input_str(&teststr);
        hasher.result(&mut hash);
        if hash[0..2] == [0u8; 2] {
            if hash[2] < 0x10 {
                let rstr = hasher.result_str();
                result.push( (
                    rstr.chars().nth(5).unwrap(),
                    rstr.chars().nth(6).unwrap(),
                    )
                );
            }
        }
    }
    result
}

fn calc_password_1<T: Digest>(mut hasher: T, input: &str) -> String {
    const CHUNK_SIZE : usize = 10_000;
    let mut index = 0;
    let mut pwchars: Vec<char> = Vec::new();
    while pwchars.len() < 8 {
        let result = calc_password_chunk(&mut hasher, &input, index..index+CHUNK_SIZE);
        pwchars.extend(result.into_iter().map(|r| r.0).collect::<Vec<_>>());
        index += CHUNK_SIZE;
    }
    let chars = pwchars.iter().map(|&x| x).collect::<String>();
    chars[0..8].to_string()
}

fn calc_password_2<T: Digest>(mut hasher: T, input: &str) -> String {
    const CHUNK_SIZE : usize = 10_000;
    let mut index = 0;
    let mut password = ['\0';8];
    let mut done = [false;8];
    'find_loop : loop {
        let result = calc_password_chunk(&mut hasher, &input, index..index+CHUNK_SIZE);
        for pwchar in result.iter()
                            .map(|t| Pwchar {
                                pos : t.0 as usize - '0' as usize,
                                chr : t.1})
                            .filter(|pwc| pwc.pos < 8) {
            if done[pwchar.pos] {continue;} // skip any repeat chars
            password[pwchar.pos] = pwchar.chr;
            done[pwchar.pos] = true;
            if done.iter().all(|&x| x) { break 'find_loop; }
        }
        index += CHUNK_SIZE;
    }
    password.iter().map(|&x| x).collect::<String>()
}

fn main() {
    println!("Enter puzzle input: ");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(num_bytes) if num_bytes > 1 => {
            let roomid = input.trim();
            println!("Searching for password for '{}' (part 1)", roomid);
            let password1 = calc_password_1(Md5::new(), &roomid);
            println!("Found password for part 1: {}", password1);
            println!("Searching for password for '{}' (part 2)", roomid);
            let password2 = calc_password_2(Md5::new(), &roomid);
            println!("Found password for part 2: {}", password2);
            std::process::exit(0);
        },
        Ok(_) => {std::process::exit(0);}
        Err(_) => {std::process::exit(1);}
    }

}

#[test]
fn test_example_1() {
    assert_eq!(calc_password_1(Md5::new(),"abc"),"18f47a30");
}

#[test]
fn test_example_2() {
    assert_eq!(calc_password_2(Md5::new(),"abc"),"05ace8e3");
}
