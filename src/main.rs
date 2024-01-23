fn add_two(n: i32) -> i32 {
    n + 2
}

fn main() {
    let num: i32 = -22;
    println!("{num:b}");
}

fn to_excess_8(num: i8) -> i8 {
    // if num > 11 {
    //     panic!("nah, number {num} too big for excess 8");
    // }
    num + 8
}

#[cfg(tests)]
mod tests;
