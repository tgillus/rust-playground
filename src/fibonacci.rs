pub fn fib(n: u32) -> u32 {
    if n < 2 {
        return n;
    }

    fib(n - 1) + fib(n - 2)
}
