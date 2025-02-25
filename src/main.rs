mod root_finding;
mod errors;
use errors::MathError;

fn main() -> Result<(), MathError>{
    let f = |x: f64| {2.0*x*x+3.0*x-5.0};
    let res = root_finding::bisection(f, -1.0, 2.0)?;
    let res2 = root_finding::secant(f, -1.0, 2.0)?;
    let res3 = root_finding::newton_raphson(f, -43.0)?;
    
    println!("Hello, world! The answer is {res}, {res2}, {res3}");
    Ok(())
}
