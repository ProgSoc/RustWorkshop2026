fn f(n: i32) -> i32 {
    if n % 2 == 1 {
        3 * n + 1
    } else {
        n / 2
    }
}

pub fn main() {
    let mut n = 17;
    println!("{n}");
    while n != 1 {
        n = f(n);
        println!("{n}");
    }
}
