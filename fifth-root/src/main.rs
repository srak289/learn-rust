fn main() {
    // Find the Fifth root of the sum of the squares of the first 100 ODD numbers only.
    let mut x: f64 = 0.0;

    for o in 0..200 {
        if o % 2 == 0 {
            x += <u8 as Into<f64>>::into(o);
        }
    }
    println!("Sum: {}", x);

    println!("Fifth root: {}", f64::powf(x, 1.0 / 5.0));
}
