fn last(list: &[i32]) -> Option<i32> {
    match list {
        [] => None,
        [x] => Some(*x),
        [_, rest @ ..] => last(rest),
    }
}

fn last_two(list: &[i32]) -> Option<(i32, i32)> {
    match list {
        [] | [_] => None,
        [x, y] => Some((*x, *y)),
        [_, rest @ ..] => last_two(rest),
    }
}

fn main() {
    println!("{:?}", last(&[]));
    println!("{:?}", last(&[1]));
    println!("{:?}", last(&[1, 2]));
    println!("{:?}", last(&[1, 2, 3]));
    println!("{:?}", last_two(&[]));
    println!("{:?}", last_two(&[1]));
    println!("{:?}", last_two(&[1, 2]));
    println!("{:?}", last_two(&[1, 2, 3]));
}
