#![allow(unused)]

use std::{collections::HashMap, io, fmt};

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

pub fn pig_latin(mut s: String) -> String
{
    let vowels: [char; 6] = ['a', 'e', 'i', 'o', 'u', 'y'];
    for l in &vowels {
        if s.starts_with(*l) {
            return s + "-hay";
        }
    }
    let first = s.remove(0);
    s + "-" + &first.to_string() + "ay"
}

pub fn company_managment()
{
    println!("Welcome to the company managment interface!");
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("\nWhat would you like to do ?");
        println!("  (1) Add a new employee to a department");
        println!("  (2) Retrieve the list of all the people in a department");
        println!("  (3) Retrieve the list of all the people in the company by department");
        println!("  (4) Quit");

        let mut action = String::new();
        io::stdin().read_line(&mut action).expect("Error: Failed to read line");

        let action: u32 = match action.trim().parse() {
            Ok(a) => a,
            Err(_) => { println!("Invalid Input: input must be an integer between 1 and 4"); continue; },
        };

        match action {
            4 => break,
            3 => {
                for (department, employees) in &company {
                    println!("{} department:", department);
                    for name in employees { println!("  - {}", name); }
                }
            },
            2 => {
                println!("Input a department:");
                let mut department = String::new();
                io::stdin().read_line(&mut department).expect("Error: Failed to read line");

                let department: String = match department.trim().parse() {
                    Ok(d) => d,
                    Err(_) => { println!("Invalid Input: input must be a valid UTF8 String"); continue; },
                };

                match company.get(&department) {
                    Some(employees) => { for name in employees { println!("  - {}", name); } },
                    None => { println!("That department doesn't exist"); continue; }
                }
            },
            1 => {
                println!("Type \"Add <NAME> to <DEPARTMENT>\":");
                let mut cmd = String::new();
                io::stdin().read_line(&mut cmd).expect("Error: Failed to read line");

                let words: Vec<&str> = cmd.trim().split_whitespace().collect();

                if words.len() != 4 || words[0] != "Add" || words[2] != "to" {
                    println!("The command is incorrect. Try again."); continue;
                }
                company.entry(words[3].to_string()).or_insert_with(Vec::new).push(words[1].to_string());
            },
            _ => println!("Invalid Input: input must be an integer between 1 and 4"),
        }
    }
    println!("Have a nice day!");
}
