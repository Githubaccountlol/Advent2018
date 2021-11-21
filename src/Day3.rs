use std::collections::HashMap;

use crate::FileInput::GetInput;

pub fn DoPart1()
{
    let counts = Countmap(&Input());

    let count = counts.iter().filter(|f| *f.1 > 1).count();

    println!("Overlapping squares: {}", count);
}

pub fn DoPart2()
{
    let map = Input();
    let counts = Countmap(&map);

    for (id, (x,y),(w,h)) in map
    {
        if 
        (x..x+w)
        .flat_map(|f| (y..y+h).map(move |g| (f,g)))
        .all(|l| counts.get(&l).unwrap() == &1)
        { println!("{}", id); }
    }
}

fn Countmap(map: &Vec<(i64, (i64,i64),(i64,i64))>) -> HashMap<(i64,i64), usize>
{
    let mut result: HashMap<(i64,i64), usize> = Default::default();

    for (_i, (x,y),(w,h)) in map.iter().cloned()
    {
        for xi in (0..w)
        {
            for yi in (0..h)
            {
                let loc = (x+xi,y+yi);
                result.insert(loc, result.get(&loc).unwrap_or(&0) + &1);
            }
        }
    }

    return result;
}

fn Input() -> Vec<(i64, (i64,i64), (i64,i64))>
{
    let input = GetInput("Day3");
    let mut map: Vec<(i64, (i64,i64), (i64,i64))> = Default::default();

    for line in input.lines()
    {
        let (i, a) = line.split_once('@').unwrap();
        let (a,b) = a.split_once(':').unwrap();
        let (a,b) = (a.trim(), b.trim());
        let (x,y) = a.split_once(',').unwrap();
        let (w,h) = b.split_once('x').unwrap();
        let x = x.parse::<i64>().unwrap();
        let y = y.parse::<i64>().unwrap();
        let w = w.parse::<i64>().unwrap();
        let h = h.parse::<i64>().unwrap();
        let i = i.replace('#', "").trim().parse::<i64>().unwrap();
        map.push((i, (x,y),(w,h)));
    }

    return map;
}