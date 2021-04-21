/***
 * Template to a Kattis solution.
 * See: https://open.kattis.com/help/rust
 * Author: Inte Viola Söderlund <violaso@kth.se>
 */

 /*One things that is slowing down your chasingsubs solution is that you're copying a lot;

    Line 8: message[index..(index + fragment.len())].to_string()
    Line 15: message[latest..(latest + fragment.len())].to_string()
    Line 17: occurance.to_string()
    Since both decrypt_message and decrypt_sample are referentially transparent, you can avoid the issue of lifetime parameters by housing all code in main.

Secondly, since all basic alphabet characters are ASCII, they only take up one byte each in a rust string. Therefore, you are able to preallocate memory by String::with_capacity(250_000). Further, to allow for indexation (instead of more cloning fragpiece.clone().nth(i).unwrap()), work with fragment and message as bytes;

    let b_message = message.as_bytes(); and
    let b_fragment = fragment.as_bytes().
    String::as_bytes doesn't copy or clone, but returns a slice of type &[u8].

Good luck!
*/

use std::io;
use std::collections::HashMap;
use std::collections::HashSet;
use std::string::String;

fn main(){
    // get standard input stream
    let input = io::stdin();

    // read message line and remove new-line
    let mut message = String::new();
    input.read_line(&mut message).expect("IO Error");
    message.pop();
    
    // read fragment line and remove new-line
    let mut fragment = String::new();
    input.read_line(&mut fragment).expect("IO Error");
    fragment.pop();
    let txt = message.as_bytes();
    let pat = fragment.as_bytes();
    let m = pat.len();
    let n = txt.len();
    let mut occurences = 0;
  
    // create lps[] that will hold the longest prefix suffix 
    // values for pattern
    let mut lps: Vec<usize> = Vec::new();
    
  
    // Preprocess the pattern (calculate lps[] array)
    
    let mut len = 0; // length of the previous longest prefix suffix

    lps[0] = 0; // lps[0] is always 0
    
    // the loop calculates lps[i] for i = 1 to M-1
    let mut i = 1;
    while i < m{
        if pat[i]== pat[len]{
            len += 1;
            lps[i] = len;
            i += 1;
        } else{
            // This is tricky. Consider the example.
            // AAACAAAA and i = 7. The idea is similar 
            // to search step.
            if len != 0{
                len = lps[len-1];
            }   // Also, note that we do not increment i here
            else{
                lps[i] = 0;
                i += 1;
            }
        }
    }
        let mut dict: HashMap<u8, u8> = HashMap::new();
        let mut uses: HashSet<u8> = HashSet::with_capacity(26);


    let mut j: usize = 0; // index for pat[]
    let mut i: usize = 0; // index for txt[]
    while i < n{
        if pat[j] == txt[i]{
            i += 1;
            j += 1;
        }
        if j == m{
            occurences += 1;
            j = lps[j-1];
        }
        // mismatch after j matches
        else {if i < n && pat[j] != txt[i]{
            // Do not match lps[0..lps[j-1]] characters,
            // they will match anyway
            if j != 0{j = lps[j-1]}
            else{i += 1}
            }
        }
    }
    if occurences == 1{

    } else {println!("{}", occurences)}
}

//
//
//
//fn decrypt_sample(b_message: &[u8], b_fragment: &[u8], adjlist: &Vec<i32>) -> bool {
//
//    let mut dict: HashMap<u8, u8> = HashMap::new();
//    let mut uses: HashSet<u8> = HashSet::with_capacity(26);
//
//    for j in 0..b_fragment.len() {
//        if dict.get(&b_fragment[j]) == None {
//            if !uses.insert(b_message[j]) {
//                return false;
//            }
//            dict.insert(b_fragment[j], b_message[j]);
//            if adjlist[b_fragment[j]as usize -97] != -1{
//                if !(b_message[j] == b_message[j + adjlist[b_fragment[j]as usize -97]as usize]){
//                    return false
//                } 
//            }
//        } else if *dict.get(&b_fragment[j]).unwrap() != b_message[j] {
//            return false;
//        }
//    }
//    true
//}
//
//fn build_adjacency_list(fragment: &[u8]) -> Vec<i32>{
//    let mut list: Vec<i32> = Vec::new();
//    for i in 0..fragment.len(){
//        
//
//    }
//    return list;
//}
//
//fn main() {
//    // get standard input stream
//    let input = io::stdin();
//
//    // read message line and remove new-line
//    let mut message = String::new();
//    input.read_line(&mut message).expect("IO Error");
//    message.pop();
//    
//    // read fragment line and remove new-line
//    let mut fragment = String::new();
//    input.read_line(&mut fragment).expect("IO Error");
//    fragment.pop();
//    
//    let mut occurance = 0;
//    let mut index = 0;
//    let mut latest = 0;
//    let b_fragment = fragment.as_bytes();
//    let adjlist = build_adjacency_list(b_fragment);
//    while index + fragment.len() <= message.len() {
//        if decrypt_sample(&message.as_bytes()[index..(index+fragment.len())], b_fragment, &adjlist) {
//            occurance += 1;
//            latest = index;
//        }
//        index += 1;
//    }
//    if occurance == 1 {
//        println!("{}", message[latest..(latest + fragment.len())].to_string());
//    }
//    else {println!("{}", occurance);}
//}