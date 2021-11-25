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
    const workerCount: usize = 5;
    const timeBase: usize = 60;
    let needs = Input();
    let mut achieved: Vec<char> = Default::default();
    let mut workers: Vec<Option<(char, usize)>> = vec![None; workerCount];
    let mut tick = 0;

    loop{
        // finish any workers which are done this tick
        if workers.iter().filter(|a| match a { Some((_,t)) => t == &tick, _ => false}).count() > 0{
            workers.iter_mut().filter(|f| f.is_some()).for_each(|a| if a.unwrap().1 == tick { achieved.push(a.unwrap().0); *a = None;});

            // completion check
            if needs.keys().all(|a| achieved.contains(a)) { break; }
        }

        // if a worker is available, try to assign some work
        if workers.iter().any(|a| a.is_none()){
            let mut pot: Vec<char> = 
            needs
            .iter()
            .filter(|(a,_)| !achieved.contains(a))
            .filter(|(a, _)| !workers.iter().filter_map(|o| *o).any(|(c,_)| a == &&c))
            .filter(|(_a,b)| b.iter().all(|b| achieved.contains(b)))
            .map(|(a,_)| *a)
            .collect();
            
            pot.sort_unstable();

            for (potentialID, worker) in (0..pot.len()).zip(workers.iter_mut().filter(|a| a.is_none()))
            {
                let char = pot[potentialID];
                let time = CalculateTime(char, timeBase) + tick;

                *worker = Some((char, time));
            }
        }

        // print!("{}\t", tick);
        // workers.iter().for_each(|f| print!("{}\t", match f { None => &'.', Some((a,_)) => a}));
        // println!();

        // if all workers busy, accelerate to first finishing time
        if workers.iter().all(|a| a.is_some()){
            tick = workers.iter().map(|a| a.unwrap().1).min().unwrap();
            continue;
        }

        tick += 1;
    }

    achieved.iter().for_each(|a| print!("{}", a));
    println!();
    println!("{}", tick);
}

fn CalculateTime(char: char, penalty: usize) -> usize
{
    let char = char.to_ascii_uppercase();

    return (char as u8 + 1 - 'A' as u8) as usize + penalty;
}

#[test]
fn calc_time_test() {
    assert_eq!(CalculateTime('A', 10), 11);
    assert_eq!(CalculateTime('a', 10), 11);
    assert_eq!(CalculateTime('D', 10), 14);
    assert_eq!(CalculateTime('e', 10), 15);
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