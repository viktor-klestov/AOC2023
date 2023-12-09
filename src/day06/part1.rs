fn main() {
    let vals = [
        (54.0_f64, 239.0),
        (70.0, 1142.0),
        (82.0, 1295.0),
        (75.0, 1253.0),
    ];
    let mut product: u64 = 1;
    for val in vals {
        let x1 = (val.0 - (val.0 * val.0 - 4.0 * val.1).sqrt()) / 2.0 + 1.0;
        let x2 = (val.0 + (val.0 * val.0 - 4.0 * val.1).sqrt()) / 2.0 - 1.0;
        product *= x2.ceil() as u64 - x1.floor() as u64 + 1;
    }
    println!("{product}");
}
