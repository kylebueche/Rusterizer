pub mod sphere;

fn quadratic_formula(a: f64, b: f64, c: f64) -> (bool, f64, f64) {
    let denominator = 2.0 * a;
    let inside_sqrt = b * b - 4.0 * a * c;
    let solution_exists = inside_sqrt >= 0.0 && denominator != 0.0;
    let root = inside_sqrt.sqrt();
    let t0 = (-b - root) / denominator;
    let t1 = (-b + root) / denominator;
    (solution_exists, t0, t1)
}