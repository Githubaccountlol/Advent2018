use std::{collections::HashMap, ops::Range};

use crate::FileInput::{self, GetInput};
use Stuff::Point;

pub fn DoPart1()
{
    Do();
}

pub fn DoPart2()
{
    Do();
}

pub fn Do()
{
    let s = Input();

    let vals: HashMap<Point<i64>, i64> = 
    (1..=300)
    .flat_map(|a| (1..=300).map(move |b| (a,b)))
    .map(|(a,b)| Point::new(a,b))
    .map(|p| (p, Step6(p,s)))
    .collect();

    println!("Squares");
    let squares = 
    SquaresAll(&vals);
    println!("Squares done");

    let bests: HashMap<i64, (Point<i64>, i64)> = 
    squares
    .map(|(s, m)| (s, m.into_iter().max_by(|(_a,b),(_c,d)| b.cmp(d)).unwrap()))
    .collect();
    
    let (bestSize, (bestPoint, bestVal)) = 
    bests
    .iter()
    .max_by(|(_s1,(_p1,v1)),(_s2,(_p2,v2))| v1.cmp(v2))
    .unwrap();

    println!("Part 1 (best size 3): {} : {}", bests.get(&3).unwrap().0, bests.get(&3).unwrap().1);
    println!("Part 2 {} : {} : size {}", bestPoint, bestVal, bestSize);
}

fn SquaresAll(vals: &HashMap<Point<i64>, i64>) -> impl Iterator<Item = (i64, HashMap<Point<i64>, i64>)>
{
    let mut sums: HashMap<Point<i64>, i64> = Default::default();

    (1..=300)
    .flat_map(|x| (1..=300).map(move |y| Point::new(x,y)))
    .for_each(|p| {ExpandSum(p, vals, &mut sums)});

    fn Calc(m: &HashMap<Point<i64>, i64>, tl: Point<i64>, br: Point<i64>) -> i64 {
        // bottom right (target)
        m.get(&br).expect(&format!("Point {} not in map", &br))
        -
        // bottom left
        m.get(&Point::new(tl.x-1,br.y)).unwrap_or(&0)
        -
        // top right
        m.get(&Point::new(br.x, tl.y-1)).unwrap_or(&0)
        +
        // top left
        m.get(&Point::new(tl.x-1, tl.y-1)).unwrap_or(&0)
    }

    fn Points(size: i64) -> impl Iterator<Item = Point<i64>>
    {
        (1..=301-size)
        .flat_map(move |a| (1..=301-size).map(move |b| Point::new(a,b)))
    }

    (1..=300)
    .map(
        move |s| 
        (s, 
            Points(s).map(
                |p|
                 (p, Calc(&sums, p, Point::new(p.x+s-1, p.y+s-1)))).collect::<HashMap<Point<i64>,i64>>()))
}

// fn CornerSum(point: Point<i64>, map: &HashMap<Point<i64>, i64>) -> i64
// {
//     (1..=point.x)
//     .flat_map(|x| (1..=point.y).map(move |y| Point::new(x,y)))
//     .map(|p| map.get(&p).unwrap())
//     .sum()
// }

fn ExpandSum(p: Point<i64>, vals: &HashMap<Point<i64>, i64>, sums: &mut HashMap<Point<i64>, i64>)
{
    let compute = 
    (1..p.y)
    .map(|y| Point::new(p.x, y))
    .map(|p| vals.get(&p).unwrap())
    .sum::<i64>()
    +
    (1..p.x)
    .map(|x| Point::new(x, p.y))
    .map(|p| vals.get(&p).unwrap())
    .sum::<i64>()
    +
    sums.get(&Point::new(p.x-1,p.y-1)).unwrap_or_else(|| {if p.x-1 >= 1 && p.y-1 >= 1 {panic!();}else {return &0;}})
    + 
    vals.get(&p).unwrap()
    ;

    sums.insert(p, compute);
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