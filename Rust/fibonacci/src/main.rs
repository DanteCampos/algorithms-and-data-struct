fn main(a:i32, b:i32, n:i32) {
    if n < 1 {
        println!("{}", a);
        return;
    }

    main(b, a + b, n - 1);
}
