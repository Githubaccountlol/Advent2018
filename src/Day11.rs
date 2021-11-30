use std::{collections::HashMap, ops::Range};

use crate::FileInput::{self, GetInput};
use Stuff::Point;

pub fn DoPart1()
{
    let s = Input();

    let map: HashMap<Point<i64>, i64> = 
    (1..=300)
    .flat_map(|a| (1..=300).map(move |b| (a,b)))
    .map(|(a,b)| Point::new(a,b))
    .map(|p| (p, Step6(p,s)))
    .collect();

    let squares: HashMap<Point<i64>, i64> = 
    (1..=298)
    .flat_map(|a| (1..=298).map(move |b| (a,b)))
    .map(|(a,b)| Point::new(a,b))
    .map(|p| (p, Square(p, &map, 3)))
    .collect();

    let best = 
    squares
    .iter()
    .max_by(|(_a,b),(_c,d)| b.cmp(d))
    .unwrap();

    println!("{} : {}", best.0, best.1);
}

pub fn DoPart2()
{
    let s = Input();

    let map: HashMap<Point<i64>, i64> = 
    (1..=300)
    .flat_map(|a| (1..=300).map(move |b| (a,b)))
    .map(|(a,b)| Point::new(a,b))
    .map(|p| (p, Step6(p,s)))
    .collect();

    let squares = 
    SquaresAll(&map, (1..=300));

    let best = 
    squares
    .iter()
    .flat_map(|(i,a)| a.iter().map(move |(p,s)| (i,p,s)))
    .max_by(|(_,_,a),(_,_,b)| a.cmp(b))
    .unwrap();

    println!("{} : {} : {}", best.0, best.1, best.2);
}

fn SquaresAll(map: &HashMap<Point<i64>, i64>, mut sizes: impl IntoIterator<Item = usize> + Iterator<Item = usize>) -> HashMap<usize, HashMap<Point<i64>, i64>>
{
    let mut new: HashMap<usize, HashMap<Point<i64>, i64>> = Default::default();
    let x = sizes.next().unwrap();
    new.insert(x, Squares(map, x));

    for size in sizes
    {
        println!("{}", size);
        
        let thing: HashMap<Point<i64>, i64> = 
        (1..=301-size)
        .flat_map(|a| (1..=301-size).map(move |b| (a as i64,b as i64)))
        .map(|(x,y)| Point::new(x,y))
        .map(
            |p|
            (p, 
            (0..size)
            .flat_map(|i| [(i as i64, (size-1) as i64), ((size-1) as i64, i as i64)])
            .map(move |(a,b)| Point::new(p.x+a,p.y+b))
            .map(|p| map.get(&p).unwrap())
            .sum::<i64>()
            + new.get(&(size-1)).unwrap().get(&p).unwrap()
            )
        )
        .collect();

        new.insert(size, thing);
    }

    return new;
}

fn Squares(map: &HashMap<Point<i64>, i64>, size: usize) -> HashMap<Point<i64>, i64>
{
    let mut squares: HashMap<Point<i64>, i64> = Default::default();

    (1..=300-(size as i64)+1)
    .map(|y| Point::new(1,y))
    .for_each(
        |pstart|
        {
            let s = Square(pstart, map, size);
            squares.insert(pstart, s);
            (2..=300-(size as i64)+1)
            .map(|x| Point::new(x,pstart.y))
            .fold(s, 
                |a,e|
                {
                    let g = a + SquareShiftRight(e, map, size);
                    squares.insert(e, g);
                    g    
                }
            );
        }
    );

    return squares;
}

fn Square(p: Point<i64>, map: &HashMap<Point<i64>, i64>, size: usize) -> i64
{
    let vals: Vec<Option<i64>> = 
    (0..size)
    .flat_map(|a| (0..size).map(move |b| (a,b)))
    .map(|(a,b)| Point::new(p.x + a as i64, p.y + b as i64))
    .map(|p| map.get(&p).cloned())
    .collect();

    if vals.iter().any(|b| b.is_none()) { panic!(); }

    return vals.iter().map(|a| a.unwrap()).sum();
}

fn SquareShiftRight(p: Point<i64>, map: &HashMap<Point<i64>, i64>, size: usize) -> i64
{
    // remove column we're leaving behind, add rightmost column of new square

    (0..size as i64)
    .map(|y| Point::new(p.x-1, p.y+y))
    .map(|p| -map.get(&p).unwrap())
    .sum::<i64>()
    +
    (0..size as i64)
    .map(|y| Point::new(p.x + size as i64-1, p.y+y))
    .map(|p| map.get(&p).unwrap())
    .sum::<i64>()
}

fn Step1(p: Point<i64>) -> i64
{
    return p.x+10;
}

fn Step2(p: Point<i64>) -> i64
{
    return Step1(p) * p.y;
}

fn Step3(p: Point<i64>, serial: i64) -> i64
{
    return Step2(p) + serial;
}

fn Step4(p: Point<i64>, serial: i64) -> i64
{
    return Step3(p, serial) * Step1(p);
}

fn Step5(p: Point<i64>, serial: i64) -> i64
{
    return (Step4(p, serial) / 100) % 10;
}

fn Step6(p: Point<i64>, serial: i64) -> i64
{
    return Step5(p,serial) - 5;
}

fn Input() -> i64
{
    let input = GetInput("Day11");

    return input.trim().parse::<i64>().unwrap();
}