use std::mem;
use std::ops::Deref;

fn main() {
    //types()
    //operators()
    //scopes()
    constants()
}

fn types() {
    // unsigned
    let a: u8 = 123; // 8 bits
    println!("a = {}", a);

    // signed
    let mut b: i16 = 456;
    println!("b = {}", b);
    b = 789;
    println!("b = {}", b);

    // auto
    let c = 12345678;
    println!("c = {}, size={} bytes", b, mem::size_of_val(&c));

    // size
    let z: isize = 123;
    let size_z = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes. Running on {}-bit OS", z, size_z, size_z * 8);

    // char
    let x: char = 'x';
    println!("x = {}, size={} bytes", x, mem::size_of_val(&x));

    // f64
    let e = 2.5;
    let _f: f32 = 2.5;

    // boolean
    let _g = false;
}

fn operators() {
    let mut a = 2 + 2 * 3;
    a = a + 5;
    a -= 3; // -= += /=
    let mod_a = a % 5;
    let a_cube = i32::pow(a, 3);
    println!("A = {}, {} mod 5 = {}, {} cube = {}", a, a, mod_a, a, a_cube);

    let f = 2.5;
    let f_pi = f64::powf(f, std::f64::consts::PI);
    let f_cube = f64::powi(f, 3);
    println!("{}^pi = {}, {} cube = {}", f, f_pi, f, f_cube);

    // | or, & and, ^ xor, ! nor
    let c = 1 | 2; // 01 or 11 = 11 = 3_10
    println!("1|2 = {}", c);

    // << >>
    let shift = 1 << 10; // 2^10
    println!("2^10 = {}", shift);

    // <, >, <=, >=, ==
    let logical = 5 < 3;
    println!("5 < 3 = {}", logical);
}

fn scopes() {
    let a = 123;
    println!("a, outside = {}", a);
    {
        let b = 456;
        println!("b = {}", b);

        let a = 789;
        println!("a, inside = {}", a);
    }

    let x = "abc";
    let x = "xyz";
    println!("x, after redeclaration = {}", x);
}

// no fixed address, like inline
const AGE: u8 = 25;
// has memory address
static YEAR: u8 = 1900;
// mutable constant
static mut MONTH: u8 = 9;

fn constants() {
    println!("Age = {}", AGE); // Will replace as 25
    //println!("Month = {}", MONTH); // Not allowed
}