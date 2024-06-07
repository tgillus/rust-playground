pub fn swap() {
    let mut x = 1;
    let mut y = 2;

    swap_ints(&mut x, &mut y);

    println!("     swap: x = {x}, y = {y}");
}

fn swap_ints(x: &mut i32, y: &mut i32) {
    let t = *x;
    *x = *y;
    *y = t;

    println!("swap_ints: x = {x}, y = {y}");
}
