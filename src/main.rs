use std::f64::consts;

fn main() {
    iterators();
    sum();
    square();
    references();

    println!("PI is {}", consts::PI);

    arrays();
    slices();
    options();
    vectors();
    strings();
}

fn iterators() {
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

fn sum() {
    let mut sum = 0.0;
    for i in 0..5 {
        sum += i as f64;
    }

    println!("Sum is {}", sum);

    let sum: i32 = (0..5).sum();
    println!("Sum is {}", sum);

    let sum: i64 = [10, 20, 30].iter().sum();
    println!("Sum is {}", sum);
}

fn square() {
    fn square(x: f64) -> f64 {
        x * x
    }

    println!("Square of {} is {}", 10.0, square(10.0));
}

fn references() {
    fn by_ref(x: &i32) -> i32 {
        println!("Reference to {}", x);
        println!("Adding one to {} is {}", *x, *x + 1);
        *x + 1
    }

    fn modifies(x: &mut f64) {
        *x = 1.0;
    }

    let i = 10;
    by_ref(&i);
    println!("{}", i);

    let mut res = 0.0;
    modifies(&mut res);
    println!("res is {}", res);
}

fn arrays() {
    let arr = [10, 20, 30, 40];
    let first = arr[0];
    println!("first is {}", first);

    for i in 0..arr.len() {
        println!("[{}] = {}", i, arr[i]);
    }

    let ints_ints = [[0, 1], [2, 4]];
    println!("ints_ints {:?}", ints_ints);
}

fn slices() {
    fn sum(values: &[i32]) -> i32 {
        let mut total = 0;
        for i in 0..values.len() {
            total += values[i];
        }
        total
    }

    let arr = [10, 20, 30, 40];
    println!("sum is {}", sum(&arr));

    let ints = [1, 2, 3, 4, 5];
    let slice1 = &ints[0..2];
    let slice2 = &ints[1..];
    println!("ints {:?}", ints);
    println!("slice1 {:?}", slice1);
    println!("slice2 {:?}", slice2);
}

fn options() {
    let ints = [1, 2, 3, 4, 5];
    let slice = &ints;
    let first = slice.get(0);
    let last = *slice.get(slice.len()).unwrap_or(&-1);

    println!("first {:?}", first);
    println!("first {} {}", first.is_some(), first.is_none());
    println!("first {}", first.unwrap());
    println!("last {:?}", last);

    for n in slice.windows(2) {
        println!("window {:?}", n);
    }

    for n in slice.chunks(2) {
        println!("chunk {:?}", n);
    }
}

fn vectors() {
    fn dump(arr: &[i32]) {
        println!("arr is {:?}", arr)
    }

    let mut v1 = Vec::new();

    v1.push(10);
    v1.push(20);
    v1.push(30);

    let first = v1[0];
    let maybe_first = v1.get(0);

    println!("v is {:?}", v1);
    println!("first is {}", first);
    println!("maybe_first is {:?}", maybe_first);

    dump(&v1);

    let slice = &v1[1..];
    println!("slice is {:?}", slice);

    let mut v2 = vec![10, 20, 30, 40];
    v2.pop();

    let mut v3 = Vec::new();
    v3.push(10);
    v3.push(20);
    v3.push(30);

    assert_eq!(v2, v3);

    v3.extend(0..2);
    assert_eq!(v3, [10, 20, 30, 0, 1]);
    assert_eq!(v3, &[10, 20, 30, 0, 1]);

    let mut v4 = vec![1, 10, 5, 1, 2, 11, 2, 40];
    v4.sort();
    v4.dedup();
    assert_eq!(v4, &[1, 2, 5, 10, 11, 40]);
}

fn strings() {
    fn dump(s: &str) {
        println!("str '{}'", s);
    }

    let text1 = "Hello, World!";
    let s1 = text1.to_string();

    dump(text1);
    dump(&s1);

    let mut s2 = String::new();
    s2.push('H');
    s2.push_str("ello,");
    s2.push(' ');
    s2 += "World!";
    s2.pop();

    assert_eq!(s2, "Hello, World");

    fn array_to_string(arr: &[i32]) -> String {
        let mut res = '['.to_string();
        for n in arr {
            res += &format!("{}, ", n.to_string());
        }
        res.pop();
        res.pop();
        res.push(']');

        res
    }

    let res = format!("hello {}", array_to_string(&[10, 20, 30]));
    assert_eq!(res, "hello [10, 20, 30]");

    let text2 = "static";
    let text3 = "dynamic".to_string();
    println!("slices {:?} {:?}", &text2[1..], &text3[2..4]);

    let multilingual = "Hi! ¡Hola! привет!";
    for ch in multilingual.chars() {
        print!("'{}'", ch);
    }
    println!("");
    println!("len {}", multilingual.len());
    println!("count {}", multilingual.chars().count());

    let maybe = multilingual.find('п');
    if maybe.is_some() {
        let hi = &multilingual[maybe.unwrap()..];
        println!("Russian hi {}", hi);
    }
}
