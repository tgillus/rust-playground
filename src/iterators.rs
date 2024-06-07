pub fn iterators() {
    for i in 0..5 {
        let even_odd = if i % 2 == 0 { "even" } else { "odd" };
        println!("{} {}", i, even_odd);
    }

    let mut iter = 0..3;
    println!("iter is {:?}", iter.next());
    println!("iter is {:?}", iter.next());
    println!("iter is {:?}", iter.next());
    println!("iter is {:?}", iter.next());

    let arr = [10, 20, 30];
    for n in arr.iter() {
        println!("{}", n);
    }

    let slice = arr;
    for n in slice {
        println!("{}", n);
    }
}
