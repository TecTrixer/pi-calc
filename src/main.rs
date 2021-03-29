use rug::Float;
use std::io::{stdout, Write};
use clap::{Arg, App};

fn main() {
    let matches = App::new("Pi Calculator")
                          .version("0.1.0")
                          .author("TecTrixer <tonihoevedes@gmail.com>")
                          .about("Calculates the value of pi using an algorithm similar to Archimedes approach")
                          .arg(Arg::with_name("DIGITS")
                               .help("Sets the number of digits of pi which are going to be calculated - the default value is 10")
                               .index(1))
                          .arg(Arg::with_name("short")
                               .short("s")
                               .long("short")
                               .help("Only prints the value of pi without a progress indicator and the number of iterations"))
                          .get_matches();
    let mut stdout = stdout();
    let num_of_digits: u32 = match matches.value_of("DIGITS").unwrap_or("10").parse::<u32>() {
        Ok(num) => num + 1,
        _ => 11
    };
    let short: bool = matches.is_present("short");
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
            "Pi: {}, Iterations: {:3}, Floating Point Precision: {}",
            pi_from_area(area.clone(), num_of_digits),
            step - 1,
            prec
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