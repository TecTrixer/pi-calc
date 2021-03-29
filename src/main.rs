use rug::Float;
use std::env;
use std::io::{stdout, Write};

const PI: &str = "3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679821480865132823066470938446095505822317253594081284811174502841027019385211055596446229489549303819644288109756659334461284756482337867831652712019091456485669234603486104543266482133936072602491412737245870066063155881748815209209628292540917153643678925903600113305305488204665213841469519415116094330572703657595919530921861173819326117931051185480744623799627495673518857527248912279381830119491298336733624406566430860213949463952247371907021798609437027705392171762931767523846748184676694051320005681271452635608277857713427577896091736371787214684409012249534301465495853710507922796892589235420199561121290219608640344181598136297747713099605187072113499999983729780499510597317328160963185950244594553469083026425223082533446850352619311881710100031378387528865875332083814206171776691473035982534904287554687311595628638823537875937519577818577805321712268066130019278766111959092164201989";

fn main() {
    // let mut stdout = stdout();
    // let args: Vec<String> = env::args().collect();
    // let num_of_digits: u32;
    // let short: bool;
    // let len = args.iter().count();
    // if len < 2 {
    //     short = false;
    //     num_of_digits = 10;
    // }
    // else if len < 3 {
    //     short = false;
    //     num_of_digits = match args[1].parse::<u32>() {
    //         Ok(num) => num,
    //         _ => 10,
    //     };
    // }
    // else {
    //     short = match &args[2] {
    //         arg => {
    //             if arg.to_owned() == "-s".to_string() || arg.to_owned() == "--short".to_string() {
    //                 true
    //             } else {
    //                 false
    //             }
    //         }
    //     };
    //     num_of_digits = match args[1].parse::<u32>() {
    //         Ok(num) => num,
    //         _ => 10,
    //     };
    // }
    // let prec: u32;
    // if num_of_digits > 100 {
    //     prec = (11.0 / 3.0 * (num_of_digits as f64)) as u32;
    // } else {
    //     prec = (15.0 / 3.0 * (num_of_digits as f64)) as u32;
    // }
    // let mut area = Float::with_val(prec, 0.5);
    // let mut step: u32 = 1;
    // let mut a = Float::with_val(prec, Float::sqrt_u(2)) / Float::with_val(prec, 2);
    // let mut b: Float;
    // if short {
    //     loop {
    //         b = b_from_a(&a, prec);
    //         area += area_of_triangle(&a, &b) * num_of_triangles(step, prec);
    //         a = new_a(a, b, prec);
    //         step += 1;
    //         if step > (1.7 * num_of_digits as f64) as u32 {
    //             break;
    //         }
    //     }
    //     println!(
    //         "{:.width$}",
    //         pi_from_area(area.clone()),
    //         width = (num_of_digits + 1) as usize
    //     );
    // } else {
    //     loop {
    //         b = b_from_a(&a, prec);
    //         area += area_of_triangle(&a, &b) * num_of_triangles(step, prec);
    //         a = new_a(a, b, prec);
    //         step += 1;
    //         if step > (1.7 * num_of_digits as f64) as u32 {
    //             break;
    //         }
    //         print!("Completed {:3.2}%\r", step as f64/ (1.7 * num_of_digits as f64) * 100.0);
    //         stdout.flush().unwrap();
    //     }
    //     println!(
    //         "Pi: {:.width$}, Step: {:3}",
    //         pi_from_area(area.clone()),
    //         step - 1,
    //         width = (num_of_digits + 1) as usize
    //     );
    // }
    for i in (1..102).step_by(2) {
        let mut prec: u32 = 10000;
        let mut max_prec: u32 = 20000;
        let mut min_prec: u32 = 0;
        let mut steps: u32 = 1000;
        let mut max_steps: u32 = 2000;
        let mut min_steps: u32 = 0;
        loop {
            let pi: &str = &calculate_pi(i as u32, prec, steps);
            if pi == &PI[..i + 1] {
                if steps - min_steps < 2 {
                    break
                }
                let temp_steps = steps;
                steps -= (steps - min_steps) / 2;
                max_steps = temp_steps;
            } else {

                let temp_steps = steps;
                steps += (max_steps - steps) / 2 + 1;
                min_steps = temp_steps;
            }
        }
        loop {
            let pi: &str = &calculate_pi(i as u32, prec, steps);
            if pi == &PI[..i + 1] {
                if prec - min_prec < 2 {
                    break
                }
                let temp_prec = prec;
                prec -= (prec - min_prec) / 2;
                max_prec = temp_prec;
            } else {

                let temp_prec = prec;
                prec += (max_prec - prec) / 2 + 1;
                min_prec = temp_prec;
            }
        }
        println!("Digits: {:5}, Precision: {:6}, Prec/Dig: {:.6}, Steps: {:6}, Steps/Dig: {:.6}", i - 1, prec, prec as f64 / (i-1) as f64, steps, steps as f64 / (i - 1) as f64);
        // let pi: &str = &calculate_pi(i as u32, 3000, 1000);
        // let equal: bool;

        // println!("{}, {}, {}", pi, &PI[..i+1], equal)
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

fn pi_from_area(area: Float) -> Float {
    area * 4
}

fn calculate_pi(num_of_digits: u32, prec: u32, steps: u32) -> String {
    let mut area = Float::with_val(prec, 0.5);
    let mut step: u32 = 1;
    let mut a = Float::with_val(prec, Float::sqrt_u(2)) / Float::with_val(prec, 2);
    let mut b: Float;
    loop {
        b = b_from_a(&a, prec);
        area += area_of_triangle(&a, &b) * num_of_triangles(step, prec);
        a = new_a(a, b, prec);
        step += 1;
        if step > steps {
            break;
        }
    }
    let mut x = format!(
        "{:.width$}",
        pi_from_area(area),
        width = num_of_digits as usize + 2
    );
    x.pop();
    x.pop();
    x
}
