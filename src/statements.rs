#![allow(dead_code)]
#![allow(unused_variables)]

pub fn statements() {
    if_statement();
    match_statement();
}

fn if_statement() {
    let temp = 35;
    if temp < 10 {
        println!("Really Cold")
    } else if temp > 40 {
        println!("Really Hot")
    } else {
        println!("Normal Temp")
    }
}

fn match_statement() {
    let country_code = 3;
    let country = match country_code {
        90 => "Turkey",
        1 => "USA",
        100..=1000 => "Other",//inclusive range
        _ => "Unknown"
    };
    println!("Country code {} is for {}", country_code, country)
}
