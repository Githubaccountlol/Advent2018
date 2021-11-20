use crate::FileInput::{self, GetInput};

pub fn DoPart1()
{
    let result = 
    Input()
    .into_iter()
    .fold(0, |a,e| a+e)
    ;

    println!("Part 1: {}", result);
}

fn Input() -> Vec<i64>
{
    return GetInput("Day1").lines().map(|f| f.parse::<i64>().unwrap()).collect();
}