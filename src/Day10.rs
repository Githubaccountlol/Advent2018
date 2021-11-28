use std::collections::HashSet;

use Stuff::Point;

use crate::FileInput::{self, GetInput};

pub fn DoPart1()
{
    let mut vec = Input();

    // assumption (not always true, but probably good enough for puzzle): winning map will be the smallest

    let mut iter = 
    (0..)
    .map(|i| { let clone = vec.clone(); Cycle(&mut vec); let ass = Assemble(&clone); let size = Size(Extents(&ass)); return (i, clone, size); })
    .peekable();

    let (mut i, mut v, mut size) = iter.next().unwrap();

    loop{
        let (_i, _nv,ns) = iter.peek().unwrap();
        if ns < &size { let next = iter.next().unwrap(); i = next.0; v = next.1; size = next.2; }
        else { break; }
    }

    println!("{}",i);
    Print(&Assemble(&v));

    // // 10681
    // for _i in (0..=10682)
    // {
    //     println!("{}", _i);
    //     let map = Assemble(&vec);
    //     let s = Size(Extents(&map));
    //     if s.0 < 80
    //     {
    //         Print(&map);
    //         println!();
    //     }

    //     Cycle(&mut vec);
    // }
}

pub fn DoPart2()
{
    DoPart1();
}

fn Cycle(vec: &mut Vec<(Point<i64>, Point<i64>)>)
{
    for (pos, vel) in vec
    {
        *pos = *pos + *vel;
    }
}

fn Assemble(vec: &Vec<(Point<i64>, Point<i64>)>) -> HashSet<&Point<i64>>
{
    return vec.iter().map(|(a,_b)| a).collect();
}

fn Print(map: &HashSet<&Point<i64>>)
{
    let (min,max) = Extents(map);

    for y in (min.y..=max.y)
    {
        for x in (min.x..=max.x){
            let c = match map.contains(&Point::new(x,y)){
                true => '#', 
                false => '.', 
            };

            print!("{}", c);
        }
        println!();
    }
}

fn Extents(map: &HashSet<&Point<i64>>) -> (Point<i64>, Point<i64>)
{
    let (x,y): (Vec<i64>, Vec<i64>) = 
    map
    .iter()
    .map(|p| (p.x,p.y))
    .unzip()
    ;

    let xmin = *x.iter().min().unwrap();
    let xmax = *x.iter().max().unwrap();
    let ymin = *y.iter().min().unwrap();
    let ymax = *y.iter().max().unwrap();
    
    return (Point::new(xmin, ymin), Point::new(xmax, ymax));
}

fn Size((min, max): (Point<i64>, Point<i64>)) -> (i64,i64)
{
    return (min.x.abs_diff(max.x) as i64, min.y.abs_diff(max.y) as i64);
}

fn Input() -> Vec<(Point<i64>, Point<i64>)>
{
    // position=< 9,  1> velocity=< 0,  2>

    let input = GetInput("Day10");

    let mut result: Vec<(Point<i64>, Point<i64>)> = Default::default();

    for line in input.lines(){
        let a = line.split_once('<').unwrap().1;
        let (p,b) = a.split_once('>').unwrap();
        let (p1,p2) = p.trim().split_once(',').map(|f| (f.0.trim().parse::<i64>().unwrap(), f.1.trim().parse::<i64>().unwrap())).unwrap();
        let pos = Point::new(p1,p2);

        let (_,b) = b.split_once('<').unwrap();
        let (b,_) = b.split_once('>').unwrap();
        let (v1,v2) = b.trim().split_once(',').map(|f| (f.0.trim().parse::<i64>().unwrap(), f.1.trim().parse::<i64>().unwrap())).unwrap();
        let vel = Point::new(v1,v2);

        result.push((pos,vel));
    }

    return result;
}