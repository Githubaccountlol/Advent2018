use std::collections::HashMap;

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
    .map(|p| (p, Square(p, &map)))
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

}

fn Square(p: Point<i64>, map: &HashMap<Point<i64>, i64>) -> i64
{
    let vals: Vec<Option<i64>> = 
    (0..3)
    .flat_map(|a| (0..3).map(move |b| (a,b)))
    .map(|(a,b)| Point::new(p.x + a, p.y + b))
    .map(|p| map.get(&p).cloned())
    .collect();

    if vals.iter().any(|b| b.is_none()) { panic!(); }

    return vals.iter().map(|a| a.unwrap()).sum();
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