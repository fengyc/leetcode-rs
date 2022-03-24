fn main() {
    println!("{}", count_numbers_with_unique_digits(2));
    println!("{}", count_numbers_with_unique_digits(3));
}

fn count_numbers_with_unique_digits(n: i32) -> i32 {
    let mut v: Vec<i32> = Vec::with_capacity(10);
    v.push(1);
    for i in 1..10 {
        // 9 * A(9, n-1)
        let mut product = 1;
        for i in 10 - n..9 {
            product *= i;
        }
        product = 9 * product;
        v.push(product + v[v.len() - 1])
    }
    if n as usize > v.len() - 1 {
        v[v.len() - 1]
    } else {
        v[n as usize]
    }
}
