use std::collections::{HashMap, HashSet};

use crate::FileInput::*;

// pub fn DoPart1()
// {
//     let points = Input();

//     let bounds = Boundary(&points);

//     // // test
//     // let bounds = (Pos(bounds.0.0,bounds.0.1),Pos(bounds.1.0,bounds.1.1));

//     let mut map: HashMap<(i64,i64), (Option<(i64,i64)>, usize)> = 
//     points.iter().cloned().map(|a| (a,(Some(a), 0))).collect();
//     let mut i = 0;
//     (1..)
//     .map(|size|
//     {
//         i = size;
//         Cycle(&points, &mut map, size, Some(&bounds))
//     })
//     .scan((), |_,e| if e {Some(())} else { None })
//     .last()
//     ;
    
//     println!("Sizes before identifying infinites");
//     for p in &points
//     {
//         let c = map.iter().filter(|(_,(o,_))| o.is_some() && o.unwrap() == *p).count();

//         println!("{:?}: {}", p, c);
//     }

//     // save map created with boundaries, non-infinites will remain the same during next step
//     let oldMap = map.clone();

//     // do same number of cycles again, this time without bounds checking
//     // infinites will be counts that have increased
//     (0..=i)
//     .for_each(
//         |x|
//         {Cycle(&points, &mut map, x, None);}
//     );

//     let mut newCounts = CountPoints(&points, &map);
//     let oldCounts = CountPoints(&points, &oldMap);

//     for p in &points
//     {
//         let c = map.iter().filter(|(_,(o,_))| o.is_some() && o.unwrap() == *p).count();

//         println!("{:?}: {}", p, c);
//     }

//     newCounts.drain_filter(|np,nc| oldCounts.get(np).unwrap() != nc);

//     println!("Finished");
//     println!("newCounts len: {}", newCounts.len());
//     newCounts
//     .iter()
//     .for_each(|(p,c)| println!("{:?} {}", p, c));

//     let (minP, minC) = newCounts.iter().min_by(|(_,a),(_,b)| a.cmp(b)).unwrap();

//     println!("Min: {:?} {}", minP, minC);
// }

pub fn DoPart1()
{
    let points = Input();
    let bounds = Boundary(&points);
    // I am not sure how far outside the rectangle created by the outermost starting points
    // we need to simulate in order to cover all points reached by any non-infinite regions
    // I thought it might be the actual bounds (no expansion needed), 
    // but I'm not sure, so doubling the bounds should be enough to guarantee coverage
    let diffs = Pos(bounds.1.0 - bounds.0.0, bounds.1.1 - bounds.0.1);
    let bounds = (bounds.0 - diffs, bounds.1 + diffs);

    let mut map: HashMap<Pos, (Option<Pos>, usize)> = Default::default();
    Fill(&points, &mut map, &bounds);

    let c1 = CountPoints(&points, &map);

    c1
    .iter()
    .for_each(|f| println!("{:?} {}", f.0, f.1));

    // if our starting bounds were sufficient, expanding the simulated region 
    // and checking for changes should be enough to reject infinite regions
    // by excluding any that are still growing at this point
    let bounds = (bounds.0 - Pos(1,1), bounds.1 + Pos(1,1));
    Fill(&points, &mut map, &bounds);

    let c2 = CountPoints(&points, &map);
    println!();
    c2
    .iter()
    .for_each(|f| println!("{:?} {}", f.0, f.1));

    let culledCounts: HashMap<Pos, usize> = 
    c2
    .iter()
    .filter(
        |(p,d)|
        c1.get(p).unwrap() == *d
    )
    .map(|(p,d)| (*p,*d))
    .collect();

    println!();
    println!("c1: {}", c1.len());
    println!("c2: {}", c2.len());
    println!("culledCounts: {}", culledCounts.len());

    let (maxP, maxD) = 
    culledCounts
    .iter()
    .max_by(|(_p1,d1),(_p2,d2)| d1.cmp(d2))
    .unwrap();

    println!("Max: {:?} {}", maxP, maxD);
}

pub fn DoPart2()
{

}

fn Square(pos: Pos, size: usize) -> impl Iterator<Item = Pos>
{
    let size = size as i64;

    let top = 
    (-size..=size)
    .map(move |a| pos + (a,size));

    let right = 
    (-size..=size)
    .rev()
    .map(move |b| pos + (size, b));

    let bot = 
    (-size..=size)
    .rev()
    .map(move |a| pos + (a,-size));

    let left = 
    (-size..=size)
    .map(move |b| pos + (-size,b));

    return top.chain(right.chain(bot.chain(left))).collect::<HashSet<Pos>>().into_iter();
}

#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(Hash)]
#[derive(Debug)]
struct Pos(i64,i64);

impl std::ops::Add for Pos{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Sub for Pos{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl std::ops::Add<(i64,i64)> for Pos{
    type Output = Self;

    fn add(self, rhs: (i64,i64)) -> Self::Output {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Add<(i64,i64)> for &Pos{
    type Output = Pos;

    fn add(self, rhs: (i64,i64)) -> Self::Output {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}

fn Distance(a: &Pos, b: &Pos) -> usize
{
    (a.0.abs_diff(b.0) + a.1.abs_diff(b.1)) as usize
}

fn Boundary(points: &HashSet<Pos>) -> (Pos,Pos)
{
    let minX = 
    points
    .iter()
    .map(|p| p.0)
    .min()
    .unwrap();

    let minY = 
    points
    .iter()
    .map(|p| p.1)
    .min()
    .unwrap();

    let maxX = 
    points
    .iter()
    .map(|p| p.0)
    .max()
    .unwrap();

    let maxY = 
    points
    .iter()
    .map(|p| p.1)
    .max()
    .unwrap();

    return (Pos(minX,minY), Pos(maxX,maxY));
}

fn Within(pos: &Pos, bounds: &(Pos,Pos)) -> bool
{
    return 
    pos.0 >= bounds.0.0 
    && pos.0 <= bounds.1.0 
    && pos.1 >= bounds.0.1 
    && pos.1 <= bounds.1.1;
}

fn Cycle(points: &HashSet<(i64,i64)>, map: &mut HashMap<(i64,i64), (Option<(i64,i64)>, usize)>, size: usize, bounds: Option<&(Pos,Pos)>) -> bool
{
    println!("{}", size);
    let mut someWithin = false;
    points
    .iter()
    .for_each(|p|
    {
        let p = Pos(p.0,p.1);
        Square(p, size)
        .filter(|a| if let Some(bounds) = bounds {Within(a, &bounds)} else {true})
        .for_each(|s|
        {
            someWithin = true;
            let d = Distance(&p, &s);

            if let Some(thing) = map.get(&(s.0,s.1)){
                let prevP = thing.0;
                let prevD = thing.1;

                if d == prevD && prevP != Some((p.0,p.1)) { map.insert((s.0,s.1), (None, d)); return; }
                else if prevD < d { return; }
            }
            map.insert((s.0,s.1), (Some((p.0,p.1)), d));
        })
    });
    if !someWithin { return false; }
    else { return true; }
}

fn Fill(points: &HashSet<Pos>, map: &mut HashMap<Pos, (Option<Pos>, usize)>, (min,max): &(Pos,Pos))
{
    let neededPoints: Vec<Pos> = 
    (min.0..=max.0)
    .flat_map(move |x| (min.1..=max.1).map(move |y| Pos(x,y)))
    .filter(|p| !map.contains_key(p))
    .collect();

    neededPoints
    .into_iter()
    .for_each(
        |p|
        {
            let (finalS, finalD) = 
            points
            .iter()
            .map(|source| (source, Distance(source, &p)))
            .fold(
                (None, None), |(prevP, prevD),(source, dist)| 
                {
                    if prevD.is_none() || prevD > Some(dist) { return (Some(source), Some(dist)); }
                    else if prevD == Some(dist) { return (None, Some(dist)); }
                    else { return (prevP, prevD); }
                }
            );
            map.insert(p, (finalS.cloned(), finalD.unwrap()));
        }
    )
}

fn CountPoints(points: &HashSet<Pos>, map: &HashMap<Pos, (Option<Pos>, usize)>) -> HashMap<Pos, usize>
{
    let mut counts: HashMap<Pos, usize> = HashMap::from_iter(points.iter().cloned().map(|a| (a,0)));

    map.iter().filter(|(_,(o,_))| o.is_some()).for_each(|(_k, v)| *counts.get_mut(&v.0.unwrap()).unwrap() += 1);

    return counts;
}

fn Input() -> HashSet<Pos>
{
    let input = GetInput("Day6");

    let mut result: HashSet<Pos> = Default::default();

    for line in input.lines()
    {
        if let Some((x,y)) = line.split_once(',').map(|(a,b)| (a.trim().parse::<i64>().unwrap(), b.trim().parse::<i64>().unwrap()))
        {
            result.insert(Pos(x,y));
        }
    }

    return result;
}