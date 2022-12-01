fn last(list: &[i32]) -> Option<i32> {
    match list {
        [] => None,
        [x] => Some(*x),
        [_, rest @ ..] => last(rest)
    }
}

fn last_two(list: &[i32]) -> Option<(i32, i32)> {
    match list {
        [] | [_] => None,
        [x, y] => Some((*x, *y)),
        [_, rest @ ..] => last_two(rest)
    }
}

fn at(list: &[i32], n: i32) -> Option<i32> {
    let tuple = (list, n);

    match tuple {
        ([], _) => None,
        ([first, _rest @ ..], 0) => Some(*first),
        ([_, rest @ ..], _) => at(rest, n - 1)
    }
}

fn length(list: &[i32]) -> i32 {
    fn count(list: &[i32], acc: i32) -> i32 {
        match list {
            [] => acc,
            [_, rest @ ..] => count(rest, acc + 1)
        }
    }

    count(list, 0)
}

fn main() {
    println!("{:?}", last(&[]) == None);
    println!("{:?}", last(&[1]) == Some(1));
    println!("{:?}", last(&[1, 2]) == Some(2));
    println!("{:?}", last(&[1, 2, 3]) == Some(3));

    println!("{:?}", last_two(&[]) == None);
    println!("{:?}", last_two(&[1]) == None);
    println!("{:?}", last_two(&[1, 2]) == Some((1, 2)));
    println!("{:?}", last_two(&[1, 2, 3]) == Some((2, 3)));

    println!("{:?}", at(&[], 0) == None);
    println!("{:?}", at(&[1], 0) == Some(1));
    println!("{:?}", at(&[1], 1) == None);
    println!("{:?}", at(&[1, 2], 0) == Some(1));
    println!("{:?}", at(&[1, 2], 1) == Some(2));
    println!("{:?}", at(&[1, 2], 2) == None);
    println!("{:?}", at(&[1, 2, 3], 0) == Some(1));
    println!("{:?}", at(&[1, 2, 3], 1) == Some(2));
    println!("{:?}", at(&[1, 2, 3], 2) == Some(3));
    println!("{:?}", at(&[1, 2, 3], 3) == None);

    println!("{:?}", length(&[]) == 0);
    println!("{:?}", length(&[1]) == 1);
    println!("{:?}", length(&[1, 2]) == 2);
    println!("{:?}", length(&[1, 2, 3]) == 3);
}
