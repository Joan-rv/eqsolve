fn differentiate(f: fn(f64) -> f64, x: f64) -> f64 {
    let e: f64 = 2.2e-16;
    let h = e.sqrt();
    let xph = x + h;
    let dx = xph - x;
    (f(xph) - f(x)) / dx
}

fn newton(f: fn(f64) -> f64, mut x: f64, error: f64) -> f64 {
    let mut x_prev = x;
    while f(x).abs() > error || (x - x_prev).abs() > error {
        x_prev = x;
        x = x - f(x) / differentiate(f, x);
    }
    x
}

fn main() {
    let f = |x: f64| {x*x - 2.0};
    println!("Solution: {}", newton(f, 2.0, 0.00001));
}
