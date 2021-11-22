use crate::FileInput::{self, GetInput};

pub fn DoPart1()
{
    let mut input = Input();

    while React(&mut input){};

    println!("Remaining: {}", input.len());
}

pub fn DoPart2()
{
    
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