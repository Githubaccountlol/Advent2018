use std::{collections::HashMap, slice::SliceIndex};

use crate::FileInput::{self, GetInput};

pub fn DoPart1()
{
    let input = Input();

    let mut twos = 0;
    let mut threes = 0;

    for line in input
    {
        let mut map: HashMap<char, i64> = Default::default();
        
        for c in line.chars(){
            let mut thing = **map.get_mut(&c).get_or_insert(&mut 0);
            thing += 1;
            map.insert(c, thing);
        }

        if map.iter().any(|a| a.1 == &2) { twos += 1;}
        if map.iter().any(|a| a.1 == &3) { threes += 1;}
    }

    println!("{} * {} = {}", twos, threes, twos * threes);
}

pub fn DoPart2()
{
    let input = Input();

    for (i,a) in input.iter().enumerate()
    {
        for b in input.iter().skip(i)
        {
            if Mismatches(a, b) == 1
            {
                let result = a.chars().zip(b.chars()).filter(|(a,b)| a == b).map(|(a,b)| a).collect::<String>();
                println!("{}", result);
                return;
            }
        }
    }
}

fn Mismatches(first: &str, second: &str) -> usize
{
    first
    .chars()
    .zip(second.chars())
    .filter(|(a,b)| a != b)
    .count()
}

fn Input() -> Vec<String>
{
    return GetInput("Day2").lines().map(|a| a.to_owned()).collect();
}