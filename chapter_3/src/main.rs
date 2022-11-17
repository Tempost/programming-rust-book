fn main() {
    assert_eq!(10 as u16, 10u16);
    println!("Hello, world!");
    println!("{}", -4_i32.abs());
    println!("{}", (-4_i32).abs());
    println!("{}_{}", '\u{CA0}', '\u{CA0}');

    // Slices
    let v: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];
    let a: [f64; 4] = [0.0, 0.707, 1.0, 0.707];
    let sv: &[f64] = &v;
    let sa: &[f64] = &a;
    
    println!("{:?}", v);
    println!("{:?}", a);
    println!("{:?}", sv);
    println!("{:?}", sa);
}
