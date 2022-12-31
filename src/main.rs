mod my_mod;
use crate::my_mod::Stack;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut count = 0;
    //initialise stacks
    let mut stacks = init(9);
    print_stacks(&stacks);

    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(x) = line {
                let v: Vec<&str> = x.split(' ').collect();
                let num_to_move  = v[1].parse::<usize>().unwrap();
                let source       = v[3].parse::<usize>().unwrap();
                let target       = v[5].parse::<usize>().unwrap();
                
                multimove(&mut stacks, num_to_move, source, target)
            }
        }
        print_stacks(&stacks);
    } else {
        println!("Unable to open file");
    }
}

fn print_stacks (stacks: &Vec<Stack<char>>) {
    println!("stacks.len: {:?}", stacks.len());
    for i in 0..stacks.len() {
        println!("Top of Stack {}: {}", i+1, *stacks[i].peek().unwrap());
    }
}
fn multimove( stacks: &mut Vec<Stack<char>>, num_to_move: usize, source: usize, target: usize) {
    let mut tmp = Stack::<char>::new();
    for i in 0..num_to_move {
        let x = stacks[source-1].pop().unwrap();
        tmp.push(x);
    }
    for i in 0..num_to_move {
        let x = tmp.pop().unwrap();
        stacks[target-1].push(x);
    }
}

fn init(len: usize) ->  Vec<Stack<char>> {
    let mut vec = Vec::with_capacity(len);
    for i in 0..len {
        let mut s = Stack::<char>::new();
        match i {
            0 => {
                    for x in ['Z', 'P', 'M', 'H', 'R'] {
                        s.push(x);
                    }          
                }
            1 => {
                for x in ['P', 'C', 'J', 'B'] {
                    s.push(x);
                }   
            }   
            2 => {
                for x in ['S', 'N', 'H', 'G', 'L', 'C', 'D'] {
                    s.push(x);
                }   
            
            }
            3 => {
                for x in ['F', 'T', 'M', 'D', 'Q','S','R','L'] {
                    s.push(x);
                }   
            }              
            4 => {
                for x in ['F', 'S', 'P', 'Q', 'B', 'T', 'Z', 'M'] {
                    s.push(x);
                }   
            }              
            5 => {
                for x in ['T', 'F', 'S', 'Z', 'B', 'G'] {
                    s.push(x);
                }   
            }              
            6 => {
                for x in ['N', 'R', 'V'] {
                    s.push(x);
                }   
            }              
            7 => {
                for x in ['P', 'G', 'L', 'T', 'D', 'V', 'C', 'M'] {
                    s.push(x);
                }   
            }              
            
            8 => {
                for x in ['W', 'Q', 'N', 'J', 'F', 'M', 'L'] {
                    s.push(x);
                }   
            }              

            _  => {
                for x in ['_'] {
                    s.push(x);
                }  
            }
        }
/* 
            [L] [M]         [M]    
        [D] [R] [Z]         [C] [L]
        [C] [S] [T] [G]     [V] [M]
[R]     [L] [Q] [B] [B]     [D] [F]
[H] [B] [G] [D] [Q] [Z]     [T] [J]
[M] [J] [H] [M] [P] [S] [V] [L] [N]
[P] [C] [N] [T] [S] [F] [R] [G] [Q]
[Z] [P] [S] [F] [F] [T] [N] [P] [W]
 1   2   3   4   5   6   7   8   9 
*/
        vec.push(s);
    }

    return vec;
}
