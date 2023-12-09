fn main() {
    let t = 54708275_f64;
    let d = 239114212951253_f64;
    let x1 = (t - (t * t - 4.0 * d).sqrt()) / 2.0 + 1.0;
    let x2 = (t + (t * t - 4.0 * d).sqrt()) / 2.0 - 1.0;
    let result = x2.ceil() as u64 - x1.floor() as u64 + 1;
    println!("{result}");
}
