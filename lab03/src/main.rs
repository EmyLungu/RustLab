fn prime(x: u16) -> bool {
    if x < 2 || x != 2 && x.is_multiple_of(2) {
        return false;
    }

    let mut d: u32 = 3;
    while d * d <= x as u32 {
        if x.is_multiple_of(d as u16) {
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
    Ascii,
    Digit,
    Base16Digit,
    Letter,
    Printable,
}

fn to_uppercase(c: char) -> Result<char, CharError> {
    if !c.is_ascii_lowercase() {
        Err(CharError::Letter)
    } else {
        Ok((c as u8 - 32) as char)
    }
}
fn to_lowercase(c: char) -> Result<char, CharError> {
    if !c.is_ascii_uppercase() {
        Err(CharError::Letter)
    } else {
        Ok((c as u8 + 32) as char)
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
    if !c.is_ascii() {
        Err(CharError::Ascii)
    } else if !c.is_numeric() {
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
        CharError::Ascii => println!("The character is not an ASCII character!"),
        CharError::Digit => println!("The character is not a digit!"),
        CharError::Base16Digit => println!("The character is not a digit in base 16!"),
        CharError::Letter => println!("The character is not a letter!"),
        CharError::Printable => println!("The character is not printable!"),
    }
}

fn main() {
    println!("{}", next_prime(7).unwrap());

    let mut num: u16 = 7;
    while next_prime(num).is_some() {
        num += 1;
    }
    if next_prime(num).is_none() {
        println!("{} nu mai are un next_prime in u16\n", num);
    }

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
        Ok(num) => println!("3 => {}", num),
        Err(e) => print_error(e),
    }
    match char_to_number('T') {
        Ok(num) => println!("T => {}\n", num),
        Err(e) => print_error(e),
    }
    match char_to_number_hex('E') {
        Ok(num) => println!("E => {}", num),
        Err(e) => print_error(e),
    }
    match char_to_number_hex('T') {
        Ok(num) => println!("T => {}\n", num),
        Err(e) => print_error(e),
    }
}
