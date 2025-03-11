fn bisection(f: fn(f64) -> f64, mut b: f64, mut a: f64, error: f64) -> f64 {
    let mut x = (a + b) / 2.0;
    while a - b > error {
        if f(x) == 0.0 {
            break;
        }
        if f(a)*f(x) < 0.0 {
            b = x;
        } else {
            a = x;
        }
        x = (a + b) / 2.0;
    }
    x
}

fn secant(f: fn(f64) -> f64, mut x0: f64, mut x1: f64, error: f64) -> f64 {
    while f(x1).abs() > error || (x1 - x0).abs() > error {
        let x_new = (x0 * f(x1) - x1 * f(x0)) / (f(x1) - f(x0));
        x0 = x1;
        x1 = x_new;
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
    let error: f64 = 5e-12;
    let precision = -(2.0 * error).log10().round() as usize;
    println!("{}", precision);
    println!("Solution with newton:\t\t{:.*}", precision, newton(f, 2.0, error));
    println!("Solution with secant:\t\t{:.*}", precision,  secant(f, 1.0, 3.0, error));
    println!("Solution with bisection:\t{:.*}", precision, bisection(f, 1.0, 3.0, error));
    println!("Real solution:\t\t\t{:.*}", precision, 2.0_f64.sqrt());
}
