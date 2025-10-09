
fn add_chars_n(mut s: String, ch: char, n: u32) -> String {
    for _ in 0..n {
        s.push(ch);
    }

    return s;
}

fn add_chars_n_ref(s: &mut String, ch: char, n: u32) {
    for _ in 0..n {
        s.push(ch);
    }
}

fn p1() {
    let mut s = String::from("");

    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + 'a' as u8) as char;
        s = add_chars_n(s, c, 26 - i);

        i += 1;
    }

    print!("{}\n\n", s);
}

fn p2() {
    let mut s = String::from("");

    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + 'a' as u8) as char;
        add_chars_n_ref(&mut s, c, 26 - i);

        i += 1;
    }

    print!("{}\n\n", s);
}

fn add_space(str: &mut String, n: u32) {
    for _ in 0..n {
        str.push(' ');
    }
}

fn add_str(str: &mut String, new_str: &str) {
    str.push_str(new_str);
}

fn add_integer(str: &mut String, mut value: i32) {
    let mut num_str = String::from("");
    let mut cnt = 0;

    let is_negative: bool = value < 0;
    if value < 0 {
        value *= -1;
    }

    while value > 0 {
        if cnt % 3 == 0 && cnt > 0 {
            num_str.push('_');
        }
        num_str.push_str(&(value % 10).to_string());
        value /= 10;
        cnt += 1;
    }

    if is_negative {
        num_str.push('-');
    }

    for ch in num_str.chars().rev() {
        str.push(ch);
    }
}

fn add_float(str: &mut String, mut value: f32) {
    let is_negative: bool = value < 0.0;
    if value < 0.0 {
        value *= -1.0;
    }

    let epsilon: f32 = 0.001; 
    let mut dot = 0;
    
    while value.fract() > epsilon {
        value *= 10.0;
        dot += 1;
    }

    let mut num_str = String::from("");
    let mut value = value.round() as i32; 
    let mut cnt = 0;

    while value > 0 {
        if cnt == dot {
            num_str.push('.');
        }
        num_str.push_str(&(value % 10).to_string());
        value /= 10;
        cnt += 1;
    }

    if is_negative {
        num_str.push('-');
    }

    let mut result_str = String::from("");
    for ch in num_str.chars().rev() {
        result_str.push(ch);
    }
    
    str.push_str(&result_str);
}

fn p3() {
    let mut res = String::from("");

    add_space(&mut res, 40);
    add_str(&mut res, "I 💚\n");
    add_space(&mut res, 40);
    add_str(&mut res, "RUST.\n\n");

    add_space(&mut res, 4);
    add_str(&mut res, "Most");
    add_space(&mut res, 12);
    add_str(&mut res, "crate");
    add_space(&mut res, 6);
    add_integer(&mut res, 306437968);
    add_space(&mut res, 11);
    add_str(&mut res, "and");
    add_space(&mut res, 5);
    add_str(&mut res, "lastest");
    add_space(&mut res, 9);
    add_str(&mut res, "is\n");
    add_space(&mut res, 9);
    add_str(&mut res, "downloaded");
    add_space(&mut res, 8);
    add_str(&mut res, "has");
    add_space(&mut res, 13);
    add_str(&mut res, "downloads");
    add_space(&mut res, 5);
    add_str(&mut res, "the");
    add_space(&mut res, 9);
    add_str(&mut res, "version");
    add_space(&mut res, 4);
    add_float(&mut res, 2.038);
    add_str(&mut res, ".\n");
    add_space(&mut res, 20);

    println!("{}", res);
}

fn main() {
    p1();
    p2();
    p3();
}