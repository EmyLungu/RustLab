fn prime(x: u16) -> bool {
    if x < 2 || x != 2 && x.is_multiple_of(2) {
        return false;
    }

    let mut d = 3;
    while d * d <= x {
        if x.is_multiple_of(d) {
            return false;
        }

        d += 2;
    }

    true
}

fn next_prime(mut x: u16) -> Option<u16> {
    loop {
        x += 1;
        if prime(x) {
            return Some(x);
        }

        if x == u16::MAX {
            break;
        }
    }
    None
}

fn add(a: u32, b: u32) -> u32 {
    if a > u32::MAX - b {
        panic!("Addition u32 overflow");
    }

    a + b
}

fn multiply(a: u32, b: u32) -> u32 {
    if a > u32::MAX / b {
        panic!("Multiplication u32 overflow");
    }
    a * b
}

#[derive(Debug)]
enum Errors {
    U32Overflow,
}

fn add2(a: u32, b: u32) -> Result<u32, Errors> {
    if a > u32::MAX - b {
        Err(Errors::U32Overflow)
    } else {
        Ok(a + b)
    }
}

fn multiply2(a: u32, b: u32) -> Result<u32, Errors> {
    if a > u32::MAX / b {
        Err(Errors::U32Overflow)
    } else {
        Ok(a * b)
    }
}

fn calculate(a: u32, b: u32, c: u32) -> Result<u32, Errors> {
    let sum = add2(a, b)?;
    let prod = multiply2(sum, c)?;
    Ok(prod)
}

enum CharError {
    // ASCII,
    Digit,
    Base16Digit,
    Letter,
    Printable,
}

fn to_uppercase(c: char) -> Result<char, CharError> {
    if !c.is_alphabetic() {
        Err(CharError::Letter)
    } else {
        Ok(c.to_ascii_uppercase())
    }
}
fn to_lowercase(c: char) -> Result<char, CharError> {
    if !c.is_alphabetic() {
        Err(CharError::Letter)
    } else {
        Ok(c.to_ascii_lowercase())
    }
}
fn print_char(c: char) -> Result<(), CharError> {
    if c.is_control() {
        Err(CharError::Printable)
    } else {
        println!("{}", c);
        Ok(())
    }
}
fn char_to_number(c: char) -> Result<u32, CharError> {
    if !c.is_ascii() || !c.is_numeric() {
        Err(CharError::Digit)
    } else {
        Ok(c.to_digit(10).unwrap())
    }
}
fn char_to_number_hex(c: char) -> Result<u32, CharError> {
    if !c.is_ascii() || c > 'F' {
        Err(CharError::Base16Digit)
    } else {
        Ok(c.to_digit(16).unwrap())
    }
}
fn print_error(err: CharError) {
    match err {
        // CharError::ASCII        => println!("The character is not an ASCII character!"),
        CharError::Digit => println!("The character is not a digit!"),
        CharError::Base16Digit => println!("The character is not a digit in base 16!"),
        CharError::Letter => println!("The character is not a letter!"),
        CharError::Printable => println!("The character is not printable!"),
    }
}

fn main() {
    println!("{}", next_prime(7).unwrap());
    println!("{}\n", next_prime(65_534).unwrap_or(0));

    println!("{}\n", add(13, 17));
    // println!("{}\n", add(u32::MAX, 1));

    println!("{}\n", multiply(6, 7));
    // println!("{}\n", multiply(u32::MAX, 2));

    match calculate(2, 5, 9) {
        Ok(ans) => {
            println!("({}+{})*{}={}", 2, 5, 9, ans);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }

    match to_uppercase('t') {
        Ok(new_c) => println!("t => {}", new_c),
        Err(e) => print_error(e),
    }
    match to_uppercase('3') {
        Ok(new_c) => println!("3 => {}\n", new_c),
        Err(e) => print_error(e),
    }
    match to_lowercase('T') {
        Ok(new_c) => println!("T => {}", new_c),
        Err(e) => print_error(e),
    }
    match to_lowercase('3') {
        Ok(new_c) => println!("3 => {}\n", new_c),
        Err(e) => print_error(e),
    }
    match print_char('T') {
        Ok(_) => {}
        Err(e) => print_error(e),
    }
    match print_char('\n') {
        Ok(_) => {}
        Err(e) => print_error(e),
    }
    match char_to_number('3') {
        Ok(new_c) => println!("3 => {}", new_c),
        Err(e) => print_error(e),
    }
    match char_to_number('T') {
        Ok(new_c) => println!("T => {}\n", new_c),
        Err(e) => print_error(e),
    }
    match char_to_number_hex('E') {
        Ok(new_c) => println!("E => {}", new_c),
        Err(e) => print_error(e),
    }
    match char_to_number_hex('T') {
        Ok(new_c) => println!("T => {}\n", new_c),
        Err(e) => print_error(e),
    }
}
