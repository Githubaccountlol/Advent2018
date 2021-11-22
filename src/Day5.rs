use std::{borrow::BorrowMut, collections::HashSet};

use crate::FileInput::{self, GetInput};

pub fn DoPart1()
{
    let mut input = Input();

    while React(&mut input){};

    println!("Remaining: {}", input.len());
}

pub fn DoPart2()
{
    let mut input = Input();

    ReactAll(&mut input);

    let chars = 
    input
    .iter()
    .map(|a| a.to_ascii_lowercase())
    .collect::<HashSet<char>>();

    let mut thing = 
    chars
    .iter()
    .map(|a| (*a, input.iter().cloned().filter(|b| b.to_ascii_lowercase() != *a).collect::<Vec<char>>()))
    .collect::<Vec<(char, Vec<char>)>>();

    thing
    .iter_mut()
    .for_each(|(_a,b)| ReactAll(b));

    // thing.iter().for_each(|(c,l)| println!("{} {}", c,l.into_iter().collect::<String>()));

    let answer = 
    thing
    .iter()
    .min_by(|(_a,b),(_c,d)| b.len().cmp(&d.len()))
    .unwrap();

    println!("{} {}", answer.0, answer.1.len());
}

fn ReactAll(string: &mut Vec<char>)
{
    while React(string){};
}

fn React(string: &mut Vec<char>) -> bool
{
    let len = string.len();
    let mut flag = false;
    for i in (0..len){
        let first = string.get(i);
        let second = string.get(i+1);

        if let Some(&first) = first{
            if let Some(&second) = second{
                if first.eq_ignore_ascii_case(&second) && first != second{
                    string.splice(i..=i+1, []);
                    flag = true;
                }
            }
        }
    }
        
    return flag;
}

fn Input() -> Vec<char>
{
    let input = GetInput("Day5").trim().to_owned();

    return input.chars().collect();
}