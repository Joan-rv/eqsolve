fn bisection(f: fn(f64) -> f64, mut b: f64, mut a: f64, error: f64) -> f64 {
    let mut x = (a + b) / 2.0;
    let mut f_a = f(a);
    let mut f_x = f(x);
    while a - b > error {
        if f_x == 0.0 {
            break;
        }
        if f_a*f_x < 0.0 {
            b = x;
        } else {
            a = x;
            f_a = f_x;
        }
        x = (a + b) / 2.0;
        f_x = f(x);
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
    use std::time::Instant;
    let f = |x: f64| {x*x - 2.0};
    let error: f64 = 5e-12;
    let precision = -(2.0 * error).log10().round() as usize;


    let now = Instant::now();
    let x_bisection = bisection(f, 1.0, 3.0, error);
    let bisection_time = now.elapsed();

    let now = Instant::now();
    let x_secant = secant(f, 1.0, 3.0, error);
    let secant_time = now.elapsed();

    let now = Instant::now();
    let x_newton = newton(f, 2.0, error);
    let newton_time = now.elapsed();

    println!("{}", precision);
    println!("Solution with bisection:\t{:.*}, took: {:?}", precision, x_bisection, bisection_time);
    println!("Solution with secant:\t\t{:.*}, took: {:?}", precision,  x_secant, secant_time);
    println!("Solution with newton:\t\t{:.*}, took: {:?}", precision, x_newton, newton_time);
    println!("Real solution:\t\t\t{:.*}", precision, 2.0_f64.sqrt());
}
