use std::io;
use std::io::prelude::*;

/* Kattis calls main function to run your solution.
Jag råkade se Filips kod, och sedan var det kört att komma på
en egen skild lösning...
*/
fn main() {

    // get standard input stream
    let input = io::stdin();

    // get input lines as iterative
    let lines = input.lock().lines().map(|_line| _line.ok().unwrap());

    for line in lines {
        if line == ".".to_string() {break;}
        let mut pattern = &line[0..1];
        let mut occurance = 1;
        let mut index = 1;
    
        while index < line.len(){
            if line.len() % pattern.len() == 0{
                if pattern == &line[index..(index + pattern.len())] {
                    occurance += 1;
                    index += pattern.len();
                }
                else {
                    index += 1;
                    occurance = 1;
                    pattern = &line[0..index];
                    if pattern.len() > line.len() / 2 {
                        break;
                    }
                }
            }
            else{
                index += 1;
                occurance = 1;
                pattern = &line[0..index];
                if pattern.len() > line.len() / 2 {
                    break;
                }
            }
        }
        println!("{}", occurance);
    }
}
