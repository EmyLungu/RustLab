fn is_prime(value: i32) -> bool {
    if value < 2 || (value != 2 && value % 2 == 0) {
        return false;
    }
    for d in 2..value / 2 {
        if value % d == 0 {
            return false;
        }
    }

    return true;
}

fn p1() {
    for num in 0..100 {
        println!("{} is {}", num, is_prime(num));
    }
}

fn is_coprime(mut a: i32, mut b: i32) -> bool {
    while b != 0 {
        let r: i32 = a % b;
        a = b;
        b = r;
    }

    return a == 1;
}

fn p2() {
    for a in 0..100 {
        for b in 0..100 {
            println!("{} si {} sunt {}", a, b, is_coprime(a, b));
        }
    }
}

fn p3() {
    for i in (2..100).rev() {
        println!("{} bottles of beer on the wall,\n{} bottles of beer.\nTake one down, pass it around,\n{} bottles of beer on the wall.\n", i, i, i - 1);
    }
    println!("1 bottle of beer on the wall,\n1 bottle of beer.\nTake one down, pass it around,\nNo bottles of beer on the wall.\n");
}

fn main() {
    p1();
    p2();
    p3();
}
