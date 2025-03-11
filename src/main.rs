fn secant(f: fn(f64) -> f64, mut x0: f64, mut x1: f64, error: f64) -> f64 {
    while f(x1).abs() > error || (x1 - x0).abs() > error {
        let x2 = (x0 * f(x1) - x1 * f(x0)) / (f(x1) - f(x0));
        x0 = x1;
        x1 = x2;
    }
    x1
}

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
    let error = 0.00001;
    println!("Solution with newton: {}", newton(f, 2.0, error));
    println!("Solution with secant: {}", secant(f, 1.0, 3.0, error));
}
