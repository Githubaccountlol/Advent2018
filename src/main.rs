#![allow(non_snake_case)]
#![allow(unused_parens)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(redundant_semicolons)]
#![allow(non_upper_case_globals)]

#![feature(option_get_or_insert_default)]
#![feature(hash_drain_filter)]
#![feature(iter_intersperse)]
#![feature(linked_list_cursors)]
#![feature(map_try_insert)]
#![feature(int_abs_diff)]

use FileInput::GetInput;

extern crate Stuff;

mod FileInput;
mod Day1;
mod Day2;
mod Day3;
mod Day4;
mod Day5;
mod Day6;
mod Day7;
mod Day8;
mod Day9;
mod Day10;
mod Day11;
mod Day12;
mod Day13;
mod Day14;
mod Day15;
mod Day16;
mod Day17;
mod Day18;
mod Day19;
mod Day20;
mod Day21;
mod Day22;
mod Day23;
mod Day24;
mod Day25;

fn main() {
    Day9::DoPart2();
}