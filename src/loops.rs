#![allow(dead_code)]
#![allow(unused_variables)]

pub fn loops() {
    while_loop();
    for_loop();
}

fn while_loop() {
    let mut x = 1;
    while x < 1000 {
        x *= 2;
        if x == 64 { continue; }
        println!("x = {}", x);
    }

    let mut y = 1;
    loop { // while true
        y *= 2;
        println!("y = {}", y);
        if y > 1337 { break; } // finish loop
    }
}

fn for_loop() {
    for x in 1..11 {
        println!("x = {}", x);
    }

    for (index, y) in (30..40).enumerate() { // indexed for loop
        println!("For index {}, y= {}", index, y);
    }
}
