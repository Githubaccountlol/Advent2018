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
    // const iters: i64 = 20;
    const iters: i64 = 50000000000;

    const printmin: i64 = -3;
    const printmax: i64 = 50;

    let (init, rules) = Input();

    println!("{} {}", init.len(), rules.len());

    let mut states: HashMap<i64, HashSet<i64>> = Default::default();
    states.insert(0, init.clone());

    let mut semistates: HashMap<i64, HashSet<i64>> = Default::default();
    states.insert(0, SemiState(&init));

    print!("{}:\t", 0);
    (printmin..printmax)
    .map(|x| states[&0].get(&x).map_or('.', |_|'#'))
    .for_each(|f| print!("{}", f));
    println!();

    let mut i = 0;

    while i < iters
    {
        i += 1;
        let state = &states[&(i-1)];
        let min = state.iter().min().unwrap() - 2;
        let max = state.iter().max().unwrap() + 2;

        let mut newState = 
        (min..=max)
        .map(|o| (o, [state.contains(&(o-2)),state.contains(&(o-1)),state.contains(&o),state.contains(&(o+1)),state.contains(&(o+2))]))
        .map(
            |(o, a)| 
            rules.iter()
            .filter(|r| r.output != a[2])
            .filter(|r| r.arr[2] == a[2])
            .filter_map(|r| r.Apply(a))
            // because the test setup does not include all the rules, this is broken for the example.
            // in the example you should assume a plant is *removed* by a rule if it is not assured by any given rules
            // our assumption was that anything that didn't match a rule should stay, but it appears that all permutations are given as rules
            .nth(0).map_or((o, a[2]), |b| (o,b))
        )
        .filter(|(_i, b)| *b)
        .map(|(i,_b)| i)
        .collect();

        // check if we've already seen this state
        // println!("{}", states[&(i-1)].union(&newState).count());

        let semi = SemiState(&newState);
        if let Some((pi,_s)) = semistates.iter().find(|(_x,a)| SemiState(a) == semi) 
        { 
            let offsetparent = Offset(&states[&pi]);
            let offsetchild = Offset(&newState);

            println!("Matched {}:{}, offsets {}:{}", pi, i, offsetparent, offsetchild); 
            let iterdistance = i - pi;
            let relativeoffset = offsetchild - offsetparent;

            // use this information to teleport us forward
            let mul = (iters - i) / iterdistance;
            let teleportdistance = mul * iterdistance;
            let oldi = i;
            i += teleportdistance;
            newState = OffsetState(&newState, relativeoffset * mul);
            println!("Teleported from {} to {} via {} steps", oldi, i, mul);
        }

        states.insert(i, newState);
        // println!("{} {}", i, states[&i].len());
        semistates.insert(i, semi);

        print!("{}:\t", i);
        (printmin..printmax)
        .map(|x| states[&i].get(&x).map_or('.', |_|'#'))
        .for_each(|f| print!("{}", f));
        println!();
    }

    println!("Sum @ {}: {}", iters, states[&iters].iter().sum::<i64>());
}

/// normalize the set so that the first entry is at zero
fn SemiState(set: &HashSet<i64>) -> HashSet<i64>
{
    let offset = Offset(set);

    return OffsetState(set, offset);
}

fn Offset(set: &HashSet<i64>) -> i64
{
    return *set.iter().min().unwrap_or(&0);
}

/// rotate a state left or right
fn OffsetState(set: &HashSet<i64>, offset: i64) -> HashSet<i64>
{
    return set.iter().map(|a| a+offset).collect();
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