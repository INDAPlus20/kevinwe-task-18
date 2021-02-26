/***
 * Template to a Kattis solution.
 * See: https://open.kattis.com/help/rust
 * Author: Inte Viola Söderlund <violaso@kth.se>
 */

use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use std::string::String;

/// Kattis calls main function to run your solution.
fn main() {
    // get standard input stream
    let input = io::stdin();

    // get input lines as iterative
    let mut lines = input
        .lock()
        .lines()
        .map(|_line| _line.ok().unwrap());
    // and get one line at a time,
    let message = lines.next().unwrap();
    let deciphered = lines.next().unwrap();
    let matches = vec![];
    //naive O(n*m) solution
    for i in 0..message.len()-deciphered.len(){
        for c in deciphered.chars(){
            
        }
    }


    if matches.len() == 1{
        println!{"{}", matches[0]};
    }
    else {
        println!("{}", matches.len());
    }

}