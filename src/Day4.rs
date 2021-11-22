use std::collections::HashMap;

use crate::FileInput::GetInput;

pub fn DoPart1()
{
    let input = Input();

    input
    .iter()
    .for_each(|f| println!("{} {:?}", f.0, f.1));

    let map = ConstructTimeTables(&input);

    // aggregate specific minutes asleep by guard
    let thing = 
    map
    .keys()
    .map(|k| 
        (
            *k, 
            (0..60)
            .map(|i| map.get(k).unwrap().iter().fold(0, |a,e| a + match e[i] {true => 0, false => 1,}))
            .collect::<Vec<usize>>()
        )
    )
    .collect::<HashMap<usize, Vec<usize>>>();

    let mostSleepyGuard = 
    thing
    .iter()
    .max_by(|(_a,b),(_c,d)| b.iter().sum::<usize>().cmp(&d.iter().sum::<usize>()))
    .unwrap();

    let mostSleepyMinute = 
    mostSleepyGuard.1
    .iter()
    .enumerate()
    .max_by(|(_a,b),(_c,d)| b.cmp(&d))
    .unwrap()
    .0;

    println!("Guard {} * minute {} = {}", mostSleepyGuard.0, mostSleepyMinute, mostSleepyGuard.0 * mostSleepyMinute);
}

pub fn DoPart2()
{
    let input = Input();

    let map = ConstructTimeTables(&input);

    map
    .iter()
    .for_each(|(a,b)| {b.iter().for_each(|b| {print!("{}\t", a); b.iter().for_each(|c| print!("{}", match c { true => '.', false => '#'})); println!();})});
    println!();

    // aggregate specific minutes asleep by guard
    let sumTables = 
    map
    .keys()
    .map(|k| 
        (
            *k, 
            (0..60)
            .map(|i| map.get(k).unwrap().iter().fold(0, |a,e| a + match e[i] {true => 0, false => 1,}))
            .collect::<Vec<usize>>()
        )
    )
    .collect::<HashMap<usize, Vec<usize>>>();

    sumTables
    .iter()
    .for_each(|(a,b)| {print!("{}\t", a); b.iter().for_each(|c| print!("{}", c)); println!();});

    // find each guard's maximum minute
    let guardsBestMinute: HashMap<usize, (usize, usize)> = 
    sumTables
    .iter()
    .map(|(id,minutes)| {let r = minutes.iter().enumerate().max_by(|(_a,b),(_c,d)| b.cmp(&d)).unwrap(); ((*id),(r.0,*r.1))})
    .collect();

    guardsBestMinute
    .iter()
    .for_each(|r| println!("Guard {} minute {} : {}", r.0, r.1.0, r.1.1));

    let r = 
    guardsBestMinute
    .into_iter()
    .max_by(|(_a,(_b,c)),(_d,(_e,f))| c.cmp(&f))
    .unwrap();

    println!("Guard {} minute {} : {} = {}", r.0, r.1.0, r.1.1, r.0 * r.1.0);
}

fn ConstructTimeTables(input: &Vec<(String, Event)>) -> HashMap<usize, Vec<Vec<bool>>>
{
    // true = awake
    let mut map: HashMap<usize, Vec<Vec<bool>>> = Default::default();
    
    let mut guard = None;
    let mut awake;
    let mut timeTable = vec![true; 60];
    use self::Event::*;
    
    for i in (0..input.len())
    {
        let (time, entry) = &input[i];

        if let Guard(id) = entry 
        {
            // push previous guard's timetable, clean up for new guard's values
            if let Some(id) = guard{
                // init guard if needed
                if map.get(&id).is_none() { map.insert(id, vec![]); }

                let timeTables = map.get_mut(&id).unwrap();
                timeTables.push(timeTable);
                timeTable = vec![true; 60];
            }

            // guards are always awake on arrival
            guard = Some(*id);
            awake = true;
        }
        else if entry == &Sleep{
            awake = false;
        }
        else if entry == &Wake{
            awake = true;
        }
        else { panic!(); }

        if !awake
        {
            let interval;
            // bug: we won't handle the last entry this way (maybe he's asleep at the end, and would need sleeping credit for the rest of the shift), 
            // it won't be counted... but that doesn't happen for our input, so let's not bother
            if let Some((ntime, _nevent)) = input.get(i+1)
            {
                interval = Interval(time, ntime);
            }
            else{
                // hacky!
                interval = Interval(time, &"9999-99-99 99:99");
            }

            // guard is asleep, push those minutes
            interval
            .for_each(|f| timeTable[f] = false);
        }
    }

    if let Some(id) = guard{
        // init guard if needed
        if map.get(&id).is_none() { map.insert(id, vec![]); }

        let timeTables = map.get_mut(&id).unwrap();
        timeTables.push(timeTable);
    }

    return map;
}

/// returns a max of (0..60)
fn Interval(first: &str, second: &str) -> std::ops::Range<usize>
{
    assert!(first.lt(second));
    let (fdate, ftime) = first.split_once(' ').unwrap();
    let (sdate, stime) = second.split_once(' ').unwrap();
    let (fhour, fmin) = ftime.split_once(':').map(|(a,b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap())).unwrap();
    let (shour, smin) = stime.split_once(':').map(|(a,b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap())).unwrap();


    let start;
    let end;
    // different days
    if (fdate != sdate){
        start = 0;
        if shour > 0 { end = 60; }
        else { end = smin; }
    }
    // same day
    else{
        if fhour > 0 { start = 60; }
        else { start = fmin; }
        if shour > 0 { end = 60; }
        else { end = smin; }
    }

    return (start..end);
}

fn Input() -> Vec<(String, Event)>
{
    // [1518-11-04 23:57] Guard #2657 begins shift
    let input = GetInput("Day4");

    let mut result = vec![];

    for line in input.lines()
    {
        let (time, rest) = line.split_once(']').unwrap();
        let time = time.replace('[', "").trim().to_owned();
        let rest = rest.trim();
        use self::Event::*;
        let event = match rest{
            "falls asleep" => Sleep, 
            "wakes up" => Wake, 
            x => {
                let a = x.split_once('#').unwrap().1.split_ascii_whitespace().nth(0).unwrap().parse::<usize>().unwrap();
                Guard(a)
            }
        };

        result.push((time, event));
    }

    result.sort_unstable_by_key(|f| f.0.clone());

    return result;
}

#[derive(Debug)]
#[derive(PartialEq, Eq)]
enum Event
{
    Guard(usize), 
    Wake, 
    Sleep, 
}