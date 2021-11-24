use std::collections::HashMap;

use crate::FileInput::{self, GetInput};

pub fn DoPart1()
{
    let needs = Input();
    let mut achieved: Vec<char> = Default::default();

    loop{
        if needs.keys().all(|a| achieved.contains(a)) { break; }

        let mut pot: Vec<char> = 
        needs
        .iter()
        .filter(|(a,_)| !achieved.contains(a))
        .filter(|(_a,b)| b.iter().all(|b| achieved.contains(b)))
        .map(|(a,_)| *a)
        .collect();
        
        pot.sort_unstable();
        
        let selected = pot[0];

        achieved.push(selected);
    }

    achieved.iter().for_each(|a| print!("{}", a));
    println!();
}

pub fn DoPart2()
{

}

fn Input() -> HashMap<char, Vec<char>>
{
    // Step C must be finished before step A can begin.
    let input = GetInput("Day7");

    let mut needs: HashMap<char, Vec<char>> = Default::default();

    for line in input.lines()
    {
        let (first,second) = line.split_once("must").unwrap();
        let first = first.split_ascii_whitespace().nth(1).unwrap();
        let first = first.trim().chars().nth(0).unwrap();

        let (_, second) = second.split_once("step").unwrap();
        let (second, _) = second.split_once("can").unwrap();
        let second = second.trim().chars().nth(0).unwrap();

        if !needs.contains_key(&first) { needs.insert(first, vec![]); }
        if !needs.contains_key(&second) { needs.insert(second, vec![]); }

        needs.get_mut(&second).unwrap().push(first);
    }

    return needs;
}