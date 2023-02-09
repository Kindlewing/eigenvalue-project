use std::time::Instant;

fn main() {
    let now = Instant::now();
    let length = 100.;

    let mut estimate: Vec<f32> = vec![1., length];
    normalize(&mut estimate);

    let tolerance: f32 = 1.0E-6;
    let mut count = 0;
    let mut lambda_0: f32 = 0.;
    let mut lambda_1: f32 = lambda_0 + 2. * tolerance;

    while (lambda_0 - lambda_1).abs() >= tolerance && count <= 1000 {
        count += 1;
    }

    let elapsed_time = now.elapsed();
    println!("Found the eigenvalue in {} seconds", elapsed_time.as_secs_f64());
}

fn normalize(vector: &mut Vec<f32>) {
    let mut m: f32 = 0.;

    for x in vector.iter() {
        m += *x as f32 * *x as f32; 
    }
    let magnitude: f32 = f32::sqrt(m);

    for x in vector.iter_mut() {
        *x /= magnitude;
    }
}
