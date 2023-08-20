use peroxide::fuga::*;
use std::collections::HashMap;

fn main() {
    let v = vec![5f64, 25f64, 10f64, 0f64, 35f64, 24f64];
    let tree = SumTree::from_vec(&v);

    // Initialize HashMap
    let mut counts = HashMap::new();
    for &k in v.iter() {
        counts.insert(k.to_bits(), 0);
    }

    // Uniform Distribution (0 ~ tree.value)
    let unif = Uniform(0f64, tree.value);

    // Sample 10000 times
    for _ in 0..10000 {
        let value = unif.sample(1)[0];
        let x = tree.sample(value).to_bits();
        *counts.entry(x).or_insert(0) += 1;
    }

    let mut df = DataFrame::new(vec![]);
    for (k, v) in counts.iter() {
        df.push(&format!("{}", f64::from_bits(*k)), Series::new(vec![*v as u64]));
    }

    df.print();

    df.write_parquet("result.parquet", CompressionOptions::Uncompressed).expect("Can't write");
}

#[derive(Debug, Clone)]
struct SumTree {
    left: Option<Box<SumTree>>,
    right: Option<Box<SumTree>>,
    value: f64,
}

impl SumTree {
    fn new(value: f64) -> SumTree {
        SumTree {
            left: None,
            right: None,
            value,
        }
    }

    fn from_vec(v: &[f64]) -> SumTree {
        // Bottom Tree
        let mut tree_vec: Vec<SumTree> = v.iter().map(|&x| SumTree::new(x)).collect();

        // Make tree from bottom to top (step by 2)
        let mut l = tree_vec.len();
        while l > 1 {
            let mut new_tree_vec: Vec<SumTree> = Vec::new();
            for i in (0..l).step_by(2) {
                if i == l - 1 {
                    new_tree_vec.push(tree_vec[i].clone());
                    break;
                }
                let left = tree_vec[i].clone();
                let right = tree_vec[i + 1].clone();
                let value = left.value + right.value;
                let new_tree = SumTree {
                    left: Some(Box::new(left)),
                    right: Some(Box::new(right)),
                    value,
                };
                new_tree_vec.push(new_tree);
            }
            tree_vec = new_tree_vec;
            l = tree_vec.len();
        }

        tree_vec.pop().unwrap()
    }

    fn sample(&self, value: f64) -> f64 {
        if self.left.is_none() && self.right.is_none() {
            self.value
        } else {
            let left = self.left.as_ref().unwrap();
            let right = self.right.as_ref().unwrap();
            let left_value = left.value;
            if value < left_value {
                left.sample(value)
            } else {
                right.sample(value - left_value)
            }
        }
    }
}


