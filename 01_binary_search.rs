use std::io;

fn main() {
    let mut y = String::new();

    io::stdin()
        .read_line(&mut y)
        .expect("Failed to read line");

    let y: f64 = y.trim().parse().expect("Parse failed");

    let mut l: f64 = 0.0;
    let mut r: f64 = y;

    for _ in 1..1000 {
        let mid = (l + r) / 2.0;
        let estimation = mid * mid * mid + mid;
        if y < estimation {
            r = mid;
        } else {
            l = mid;
        }
    }

    print!("The answer is {l}")
}