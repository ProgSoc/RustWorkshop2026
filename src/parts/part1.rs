fn f(n: i32) -> i32 {
    todo!()
}

pub fn main() {
    let mut n = 17;
    println!("{n}");
    while n != 1 {
        n = f(n);
        println!("{n}");
    }
}
