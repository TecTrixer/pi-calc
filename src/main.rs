use rug::Float;
use std::env;
use std::io::{stdout, Write};

fn main() {
    let mut stdout = stdout();
    let args: Vec<String> = env::args().collect();
    let num_of_digits: u32;
    let short: bool;
    let len = args.iter().count();
    if len < 2 {
        short = false;
        num_of_digits = 11;
    }
    else if len < 3 {
        short = false;
        num_of_digits = match args[1].parse::<u32>() {
            Ok(num) => num + 1,
            _ => 11,
        };
    }
    else {
        short = match &args[2] {
            arg => {
                if arg.to_owned() == "-s".to_string() || arg.to_owned() == "--short".to_string() {
                    true
                } else {
                    false
                }
            }
        };
        num_of_digits = match args[1].parse::<u32>() {
            Ok(num) => num + 1,
            _ => 11,
        };
    }
    let prec: u32;
    let steps: u32;
    if num_of_digits > 200 {
        prec = (10.5 / 3.0 * (num_of_digits as f64)) as u32;
        steps = (1.67 * num_of_digits as f64) as u32;
    } else {
        prec = (18.0 / 3.0 * (num_of_digits as f64)) as u32;
        steps = 3 * num_of_digits;
    }
    let mut area = Float::with_val(prec, 0.5);
    let mut step: u32 = 1;
    let mut a = Float::with_val(prec, Float::sqrt_u(2)) / Float::with_val(prec, 2);
    let mut b: Float;
    if short {
        loop {
            b = b_from_a(&a, prec);
            area += area_of_triangle(&a, &b) * num_of_triangles(step, prec);
            a = new_a(a, b, prec);
            step += 1;
            if step > steps {
                break;
            }
        }
        println!(
            "{}",
            pi_from_area(area.clone(), num_of_digits),
        );
    } else {
        loop {
            b = b_from_a(&a, prec);
            area += area_of_triangle(&a, &b) * num_of_triangles(step, prec);
            a = new_a(a, b, prec);
            step += 1;
            if step > steps {
                break;
            }
            print!("Completed {:3.2}%\r", step as f64/ (1.7 * num_of_digits as f64) * 100.0);
            stdout.flush().unwrap();
        }
        println!(
            "Pi: {}, Step: {:3}",
            pi_from_area(area.clone(), num_of_digits),
            step - 1,
        );
    }

}

fn area_of_triangle(a: &Float, b: &Float) -> Float {
    0.5 * a.to_owned() * b.to_owned()
}
fn num_of_triangles(step: u32, prec: u32) -> Float {
    Float::with_val(prec, Float::i_pow_u(2, step))
}

fn b_from_a(a: &Float, prec: u32) -> Float {
    Float::with_val(prec, 1) - (Float::with_val(prec, 1) - a * a).sqrt()
}

fn new_a(previous_a: Float, previous_b: Float, prec: u32) -> Float {
    Float::with_val(
        prec,
        0.5 * (previous_a.clone() * previous_a + previous_b.clone() * previous_b).sqrt(),
    )
}

fn pi_from_area(area: Float, digits: u32) -> String {
    let pi = (area * 4) as Float;
    let mut pi_string: String = format!("{:.width$}", pi, width=(digits + 1) as usize);
    pi_string.pop();
    pi_string
}