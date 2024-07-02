

fn create_cumulative_distribution(probs: &HashMap<&str, f64>) -> Vec<(&str, f64)> {
    let mut cumulative = Vec::new();
    let mut sum = 0.0;

    
    for (&key, &prob) in probs.iter() {
        sum += prob;
        cumulative.push((key, sum));
    }

    cumulative
}
