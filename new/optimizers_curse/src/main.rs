use peroxide::fuga::*;
use rayon::prelude::*;
use dialoguer::{Input, Select, theme::ColorfulTheme};

fn main() {
    let n = 10000usize;

    let input = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select mode")
        .items(&vec!["Obtain expected max Q", "Write max Q distribution"])
        .interact()
        .unwrap();

    if input == 0 {
        let k = Input::<usize>::with_theme(&ColorfulTheme::default())
            .with_prompt("Input k")
            .default(1)
            .interact()
            .unwrap();

        let result = experiment(n, k);
        println!("Expected max Q = {} for k = {}", result, k);
    } else {
        let k_vec = vec![1usize, 2, 5, 10];
        let mut df = DataFrame::new(vec![]);

        for k in k_vec {
            let dist = experiment_dist(n, k);
            df.push(&format!("{}", k), Series::new(dist));
        }

        df.print();

        df.write_parquet("distribution.parquet", CompressionOptions::Uncompressed).unwrap();
    }
}

#[allow(non_snake_case)]
fn expected_max_Q(k: usize) -> f64 {
    let normal = Normal(0, 1);
    normal.sample(k).max()
}

fn experiment(n: usize, k: usize) -> f64 {
    (0 .. n)
        .into_par_iter()
        .map(|_| expected_max_Q(k))
        .sum::<f64>() / (n as f64)
}

fn experiment_dist(n: usize, k: usize) -> Vec<f64> {
    (0 .. n)
        .into_par_iter()
        .map(|_| expected_max_Q(k))
        .collect::<Vec<_>>()
}
