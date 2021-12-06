use std::collections::{HashMap, HashSet};

use crate::FileInput::{self, GetInput};

pub fn DoPart1()
{
    let (init, rules) = Input();

    println!("{} {}", init.len(), rules.len());

    let mut states: HashMap<i64, HashSet<i64>> = Default::default();
    states.insert(0, init);

    for i in (1..=20)
    {
        let state = &states[&(i-1)];
        let min = state.iter().min().unwrap() - 2;
        let max = state.iter().max().unwrap() + 2;

        let newState = 
        (min-2..=max+2)
        .map(|o| (o, [state.contains(&(o-2)),state.contains(&(o-1)),state.contains(&o),state.contains(&(o+1)),state.contains(&(o+2))]))
        .filter_map(|(i, a)| rules.iter().filter_map(|r| r.Apply(a)).nth(0).and_then(|f| Some((i,f))))
        .filter(|(_i, b)| *b)
        .map(|(i,_b)| i)
        .collect();

        states.insert(i, newState);
        println!("{} {}", i, states[&i].len());
    }

    println!("Sum @ 20: {}", states[&20].iter().sum::<i64>());
}

pub fn DoPart2()
{

}

fn Input() -> (HashSet<i64>, Vec<Rule>)
{
    // initial state: #..#.#..##......###...###
    // ...## => #

    let input = GetInput("Day12");
    let mut lines = input.lines();
    
    let init = lines.next().unwrap().strip_prefix("initial state:").unwrap().trim();
    let init: HashSet<i64> = init.chars().enumerate().filter_map(|(i,c)| match c { '#' => Some(i), '.' => None, _ => panic!()}).map(|i| i as i64).collect();

    (
        init
        ,
        lines
        .filter_map(|s| s.split_once("=>").map(|(a,b)| (a.trim(),b.trim())))
        .map(|(l,r)| (l.chars().into_iter(), r.chars().nth(0).unwrap()))
        .map(|(l,r)| (l.map(move |a| match a {'#' => true, '.' => false, _ => panic!()}), r))
        .map(|(mut l,r)| (
            [l.next().unwrap(),l.next().unwrap(),l.next().unwrap(),l.next().unwrap(),l.next().unwrap()]
            , 
            match r {'#' => true, '.' => false, _ => panic!()}
        ))
        .map(|(l,r)| Rule{arr: l, output: r})
        .collect()
    )
}

struct Rule{
    arr: [bool; 5], 
    output: bool, 
}

impl Rule{
    fn Apply(&self, data: [bool; 5]) -> Option<bool>
    {
        if data == self.arr { return Some(self.output); }
        else { return None; }
    }
}