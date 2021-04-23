/**
 * Solution for Kattis problem ´Chasing Subs´
 * 
 * See: https://open.kattis.com/problems/chasingsubs
 * Author: Eskil Queseth <eskilq>
 */

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::io::{BufRead, stdin};
use std::num::Wrapping;

fn main() {
    let stdin = stdin();
    let mut lines = stdin
        .lock()
        .lines()
        .map(|_line| _line.unwrap());
    
    let text = lines
        .next().unwrap()
        .chars()
        .map(|_c| _c as u8)
        .collect::<Vec<_>>();
    let pattern = lines
        .next().unwrap()
        .chars()
        .map(|_c| _c as u8)
        .collect::<Vec<_>>();
    
    if pattern.len() > text.len() {
        println!("0");
        return;
    }
    
    let mut buckets: Vec<(usize, Wrapping<u64>)> = Vec::new();
    let mut dict = HashMap::new();
    
    for (i, c) in pattern.iter().enumerate() {
        
        let num = Wrapping(P.overflowing_pow((pattern.len() - i - 1) as u32).0);
        
        match dict.entry(*c) {
            Entry::Vacant(_entry) => {
                _entry.insert(buckets.len());
                buckets.push((i, num));
            }
            Entry::Occupied(_entry) => {
                buckets[*_entry.get()].1 += num;
            }
        }
    }
    
    let mut matches = Vec::new();
    
    let iter = HashIterator::new(&text, pattern.len());
    
    'outer: for (rolling_hash, text_chunk) in iter.zip(text.windows(pattern.len())) {
        let mut uses = [false; 256];
        let mut sum = Wrapping(0);
        for bucket in &buckets {
            let c = text_chunk[bucket.0] as u64;
            if uses[c as usize] {
                continue 'outer;
            } else {
                uses[c as usize] = true;
                sum += Wrapping(c) * bucket.1;
            }
        }
        
        if sum == rolling_hash {
            matches.push(text_chunk);
        }
    }
    
    if matches.len() == 1 {
        println!("{}", matches[0].iter().map(|e| (*e) as char).collect::<String>());
    } else {
        println!("{}", matches.len());
    }
}

const P: u64 = 69;

struct HashIterator<'a> {
    text: &'a [u8],
    rolling_hash: Wrapping<u64>,
    window_size: usize,
    position: usize,
}

impl<'a> HashIterator<'a> {
    fn new(text: &'a [u8], pattern_len: usize) -> Self {
        Self {
            text,
            rolling_hash: hash(&text[0..pattern_len]),
            window_size: pattern_len,
            position: 0,
        }
    }
}

impl<'a> Iterator for HashIterator<'a> {
    type Item = Wrapping<u64>;
    
    fn next(&mut self) -> Option<Self::Item> {
        let out;
        
        if self.position <= self.text.len() - self.window_size {
            out = Some(self.rolling_hash);
            
            if self.position != self.text.len() - self.window_size {
                self.rolling_hash = remove_first(self.rolling_hash, &self.text[self.position..self.position + self.window_size]);
                self.rolling_hash = self.rolling_hash * Wrapping(P) + Wrapping(self.text[self.position + self.window_size] as u64);
            }
            
            self.position += 1;
        } else {
            out = None;
        }
        
        out
    }
}

fn hash(to_hash: &[u8]) -> Wrapping<u64> {
    to_hash.iter()
        .map(|e| Wrapping(*e as u64))
        .enumerate()
        .map(|e| Wrapping(P.overflowing_pow((to_hash.len() - e.0 - 1) as u32).0) * e.1)
        .fold(Wrapping(0), |acc, e| acc + e)
}

fn remove_first(old_hash: Wrapping<u64>, hashed: &[u8]) -> Wrapping<u64> {
    old_hash - Wrapping(hashed[0] as u64) * Wrapping(P.overflowing_pow((hashed.len() - 1) as u32).0)
}