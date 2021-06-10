mod guessing_game;
mod common_concepts;
mod ownership;
mod structs;
mod enums;
mod collections;

fn main()
{
    let v: Vec<i32> = vec![1, 1, 2, 3, 4, 5, 6, 1, 5];
    println!("Stats of the data: {:?}", collections::stats(v));
    let word = "first";
    println!("pig latin of {} is {:?}", word, collections::pig_latin(word.to_string()));
}
