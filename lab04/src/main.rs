use std::{fs, io};

fn p1(filepath: &str) -> Result<(), io::Error> {
    let s = fs::read_to_string(filepath)?;

    let mut line = String::new();
    let mut longest_bytes = String::new();
    let mut longest_chars = String::new();

    for c in s.chars() {
        line.push(c);
        if c == '\n' {
            if line.ends_with('\n') {
                line.pop();
            }
            if line.len() > longest_bytes.len() {
                longest_bytes = line.clone();
            }
            if line.chars().count() > longest_chars.chars().count() {
                longest_chars = line.clone();
            }
            line.clear();
        }
    }

    println!("The longest line by bytes is: {}", longest_bytes);
    println!("The longest line by chars is: {}\n", longest_chars);

    Ok(())
}

fn p2(s: &str) -> Result<String, io::Error> {
    let mut new_s: String = String::new();
    for ch in s.chars() {
        if !ch.is_ascii_alphabetic() {
            return Err(io::Error::other("Error: Un caracter nu este ASCII!\n"));
        }

        let start_ch: u8 = if ch.is_ascii_uppercase() { b'A' } else { b'a' };
        let new_ch: char = ((ch as u8 - start_ch + 13) % 26 + start_ch) as char;
        new_s.push(new_ch);
    }

    Ok(new_s)
}

fn p3(input_filepath: &str, output_filepath: &str) -> Result<(), io::Error> {
    let text = fs::read_to_string(input_filepath)?;
    
    let mut output_s = String::new();
    for word in text.split(" ") {
        match word {
            "pt" | "ptr" => output_s.push_str("pentru"),
            "dl" => output_s.push_str("domnul"),
            "dna" => output_s.push_str("doamna"),
            _ => output_s.push_str(word),
        }
        output_s.push(' ');
    }

    fs::write(output_filepath, &output_s)?;

    println!("{} => {}", text, output_s);
    Ok(())
}

fn p4() -> Result<(), io::Error> {
    let s = fs::read_to_string("/etc/hosts")?;

    let mut line = String::new();

    for c in s.chars() {
        line.push(c);
        if c == '\n' {
            line = line.trim().to_string();

            if line.is_empty() || line.starts_with("#") {
                line.clear();
                continue;
            }

            let mut parts = line.split_whitespace();
            let ip_address = parts.next();
            let hostname = parts.next();

            if ip_address.is_some() && hostname.is_some() {
                println!("{} => {}", hostname.unwrap(), ip_address.unwrap());
            }
            line.clear();
        }
    }

    Ok(())
}

fn main() {
    println!("p1");
    match p1("p1.txt") {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }
    match p1("p1error.txt") {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }

    println!("\np2");
    match p2("abCdFGHiJklmNOpqrStUVXyZ") {
        Ok(new_s) => println!("abCdFGHiJklmNOpqrStUVXyZ => {}", new_s),
        Err(e) => println!("{}", e),
    }
    match p2("Hel🦀o") {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }

    println!("p3");
    match p3("p3input.txt", "p3output.txt") {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }
    match p3("p3error.txt", "p3output.txt") {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }

    println!("\np4");
    match p4() {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }
}
