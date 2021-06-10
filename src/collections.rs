#![allow(unused)]

use std::collections::HashMap;

pub fn vectors()
{
    let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];
    v.push(4);
    v.push(5);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    for i in &v {
        println!("{}", i);
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

pub fn strings()
{
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let mut s = String::from("lo");
    s.push('l');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
}

pub fn hashmaps()
{
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);


    scores.clear();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);


    scores.clear();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}

pub fn stats(mut list: Vec<i32>) -> HashMap<String, f64>
{
    let mut output = HashMap::new();

    let sum: i32 = list.iter().sum();
    let size = list.len();
    let mean = f64::from(sum) / f64::from(size as i32);
    output.insert("Mean".to_string(), mean);

    output.insert("Median".to_string(), f64::from(*list.select_nth_unstable(size/2).1));

    let mut occurences: HashMap<i32, i32> = HashMap::new();
    let mut mode = 0;
    let mut mode_occurence = 0;

    for value in &list {
        let count = occurences.entry(*value).or_insert(0);
        *count += 1;
        if *count >= mode_occurence {
            mode_occurence = *count;
            mode = *value;
        }
    }
    output.insert("Mode".to_string(), f64::from(mode));

    output
}