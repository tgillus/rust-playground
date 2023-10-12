fn main() {
    basics();
}

fn basics() {
    let mut sum = 0.0;
    for i in 0..5 {
        let even_or_odd = match i % 2 {
            0 => "even",
            _ => "odd",
        };
        println!("{} is {}", i, even_or_odd);
        sum += i as f64;
    }
    println!("sum is {}", sum);

    fn square(x: f64) -> f64 {
        x * x
    }
    println!("square is {}", square(2.0));

    fn by_ref(num: &i32) -> i32 {
        *num + 1
    }
    let x = 10;
    let result = by_ref(&x);
    println!("{}", result);
    let result = by_ref(&41);
    println!("{}", result);

    fn modifies(num: &mut f64) {
        *num = 1.0;
    }
    let mut result = 0.0;
    modifies(&mut result);
    println!("{}", result);
}
