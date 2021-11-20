use std::collections::HashSet;

use crate::FileInput::{self, GetInput};

pub fn DoPart1()
{
    let result = 
    Input()
    .into_iter()
    .fold(0, |a,e| a+e)
    ;

    println!("Part 1: {}", result);
}

pub fn DoPart2()
{
    let mut seen: HashSet<i64> = Default::default();
    let input = Input();    
    let mut freq = 0;

    for item in input.iter().cycle()
    {
        freq += *item;
        if !seen.insert(freq) { break; }
    }

    println!("First repeat: {}", freq);
}

fn Input() -> Vec<i64>
{
    return GetInput("Day1").lines().map(|f| f.parse::<i64>().unwrap()).collect();
}