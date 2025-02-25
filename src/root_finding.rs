use crate::errors::MathError;

pub fn bounds_check<T: Fn(f64) -> f64>(f: T, min: f64, max: f64) -> Result<(), MathError>{
    let bounds = f(min)*f(max);
    match bounds{
        bounds if bounds > 0.0 => Err(MathError::BoundsError),
        _ => Ok(())
    }
}

pub fn bisection<T: Fn(f64) -> f64>(f: T, mut min: f64, mut max: f64) -> Result<f64, MathError>{
    bounds_check(&f, min, max)?;
    let mut midpoint = (max-min)/2.0;
    for _ in 1..50 {
        let midpoint_result = f(midpoint);
        if midpoint_result > 0.0{
            max = midpoint;
        } else {
            min = midpoint;
        }
        midpoint = (max-min)/2.0;
    }
    Ok(midpoint)
}
pub fn secant<T: Fn(f64) -> f64>(f: T, mut min: f64, mut max: f64) -> Result<f64, MathError>{
    bounds_check(&f, min, max)?;
    let mut zero_point = 0.0;
    for _ in 1..25 {
        let m = (f(max)-f(min))/(max-min);
        let b = f(max)-m*max;
        zero_point = -b/m;
        if f(zero_point) > 0.0{
            max = zero_point;
        } else {
            min = zero_point;
        }
    }
    Ok(zero_point)
}
pub fn derivative<T: Fn(f64) -> f64>(f: T, x: f64, half_eps: f64) -> Result<f64, MathError>{
    Ok((f(x+half_eps)-f(x-half_eps))/(half_eps*2.0))
}
pub fn newton_raphson<T: Fn(f64) -> f64>(f: T, start_point: f64) -> Result<f64, MathError>{
    let mut zero_point = start_point;
    for _ in 1..10 {
        let m = derivative(&f, zero_point, 0.0001)?;
        let b = f(zero_point)-m*zero_point;
        zero_point = -b/m;
    }
    Ok(zero_point)
}