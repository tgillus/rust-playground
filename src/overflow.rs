// Change types of parameters to i16 and ensure the arguments are large to
// cause and overflow.

pub fn interproduct(a: i32, b: i32, c: i32) -> i32 {
    a * b + b * c + c * a
}
