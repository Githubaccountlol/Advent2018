use crate::FileInput::{self, GetInput};

pub fn DoPart1()
{
    let nodes = Input();
    
    fn sum<'a>(e: impl IntoIterator<Item = &'a Node>) -> i64 
    { 
        e
        .into_iter()
        .map(
            |g| -> i64 
            {
                g.meta.iter().sum::<i64>() + sum(&g.children)
            }
        )
        .sum()
    };

    let answer = sum(&[nodes]);
    println!("{}", answer);
}

pub fn DoPart2()
{

}

fn Input() -> Node
{
    let input = GetInput("Day8");

    let mut nums: Vec<i64> = input.split_ascii_whitespace().filter_map(|a| a.parse::<i64>().map_or(None, |f| Some(f))).collect();

    return Node::Parse(&mut nums);
}

struct Node
{
    pub children: Vec<Node>, 
    pub meta: Vec<i64>, 
}

impl Node
{
    fn Parse(nums: &mut Vec<i64>) -> Self
    {
        let childCount = nums[0] as usize;
        let metaCount = nums[1] as usize;
        nums.splice((0..2), []);
        
        let children = 
        (0..childCount)
        .map(|_| Self::Parse(nums))
        .collect();
        
        let meta = 
        (0..metaCount)
        .map(|i| nums[i])
        .collect();
        nums.splice((0..metaCount), []);

        return Self{children, meta};
    }
}