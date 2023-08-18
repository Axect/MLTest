use peroxide::fuga::*;
use rayon::prelude::*;
use dialoguer::{Input, theme::ColorfulTheme};

fn main() {
    let n = 10000usize;
    let k = Input::<usize>::with_theme(&ColorfulTheme::default())
        .with_prompt("Input k")
        .default(1)
        .interact()
        .unwrap();

    let result = experiment(n, k);
    println!("Expected max Q = {} for k = {}", result, k);
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
