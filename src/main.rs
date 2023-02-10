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
        let mut next_estimate = matrix_vector();
    }

    let elapsed_time = now.elapsed();
    println!("Found the eigenvalue in {} seconds", elapsed_time.as_secs_f64());
}

fn load_matrix(input_file: String) {
    let mut f = BufReader::new(File::open("input.txt").unwrap());
    let mut num_line = String::new();
    f.read_line(&mut num_line).unwrap();
    let n: usize = num_line[1..].trim().parse().unwrap();

    let arr: Vec<Vec<f64>> = f.lines()
        .take(n)
        .map(|l| l.unwrap().split(char::is_whitespace)
             .take(n)
             .map(|number| number.parse().unwrap())
             .collect())
        .collect();

    println!("{:?}", arr);
}


fn matrix_vector() {
    
}



fn normalize(vector: &mut Vec<f32>) {
    let v = vector.clone();
    for x in vector.iter_mut() {
        *x /= magnitude(&v);
    }
}

fn magnitude(vector: &Vec<f32>) -> f32 {
    let mut magnitude: f32 = 0.;
    for x in vector.iter() {
        magnitude += *x as f32 * *x as f32; 
    }
    f32::sqrt(magnitude)
}
