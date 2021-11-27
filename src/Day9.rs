use std::collections::{LinkedList, linked_list::{Cursor, CursorMut}};

use Stuff::{Modulo, UsefulCursor};

use crate::FileInput::{self, GetInput};

pub fn DoPart1()
{
    let (playercount, endAt) = Input();

    let scores = Play(playercount, endAt);

    let (player, score) = 
    scores
    .iter()
    .enumerate()
    .max_by(|(_a,b),(_c,d)| b.cmp(d))
    .unwrap();

    println!("Player {} : {}", player, score);
}

pub fn DoPart2()
{
    let (playercount, endAt) = Input();
    let endAt = endAt * 100;

    let scores = Play(playercount, endAt);

    let (player, score) = 
    scores
    .iter()
    .enumerate()
    .max_by(|(_a,b),(_c,d)| b.cmp(d))
    .unwrap();

    println!("Player {} : {}", player, score);
}

fn Play(playercount: usize, endAt: usize) -> Vec<usize>
{
    let mut scores = vec![0;playercount];
    let mut marbles = LinkedList::new();
    marbles.push_front(0);
    let mut current = marbles.cursor_front_mut();
    let mut currentPlayer = 0;

    for m in (1..=endAt)
    { 
        DoTurn(m, currentPlayer, &mut current, &mut scores);
        currentPlayer = (currentPlayer + 1) % playercount; 
    };

    return scores;
}

fn DoTurn(marble: usize, player: usize, current: &mut CursorMut<usize>, scores: &mut Vec<usize>)
{
    if marble % 23 == 0 { 
        let mut score = marble;
        (0..7).for_each(|_| current.prev());
        let other = current.remove_current().unwrap();
        score += other;

        scores[player] += score;
    }
    else
    {
        current.next();
        current.insert_after(marble);
        current.next();
    }
}

fn Input() -> (usize,usize)
{
    // 439 players; last marble is worth 71307 points

    let input = GetInput("Day9");

    let mut iter = input.split_ascii_whitespace().filter_map(|f| f.parse::<usize>().ok());

    return (iter.next().unwrap(), iter.next().unwrap());
}