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
    let mut converted_strings = vec![];
    let mut converted_deciphered = vec![];
    let mut matches = vec![];

    //shamelessly stolen
    let alphabet = (b'A'..=b'z')           // Start as u8
        .map(|c| c as char)            // Convert all to chars
        .filter(|c| c.is_alphabetic()) // Filter only alphabetic chars
        .collect::<Vec<_>>();          // Collect as Vec<char>


    //convert the message into an amount of strings equal to the length of the deciphered message
    for modulo in 0..26{
        let mut temp = String::from("");
        let mut char_values = HashMap::new();
        let mut i = 0;
        //matching char to value
        for c in &alphabet{
            char_values.insert(c, (i+modulo)%26);
            i += 1;
        }
        //pushing matched values to temp vec
        for c in message.chars(){
            temp.push_str(&char_values.get_mut(&c).unwrap().to_string());
        }
        //we also want to convert the deciphered message to the same format
        for c in deciphered.chars(){
            
            converted_deciphered.push_str(&char_values.get_mut(&c).unwrap().to_string());
        }
        converted_strings.push(temp);
    }
        

    for string in converted_strings{
        for deciphered in &converted_deciphered{
            if string.contains(&deciphered){
                //fuck i need to know where this is
                matches.push("temp");
            }
        }
    }
    if matches.len() == 1{
        println!{"{}", matches[0]};
    }
    else {
        println!("{}", matches.len());
    }
}