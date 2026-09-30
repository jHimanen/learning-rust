pub fn sum(values: &[f64]) -> f64 {
    let mut result = 0.0;
    for &x in values {
        result += x;
    }
    result
}

pub fn mean(values: &[f64]) -> f64 {
    sum(values) / values.len() as f64
}

pub fn min(values: &[f64]) -> f64 {
    let mut result = f64::INFINITY;
    for &x in values {
        if x < result {
            result = x;
        }
    }
    result
}

pub fn max(values: &[f64]) -> f64 {
    let mut result = f64::NEG_INFINITY;
    for &x in values {
        if x > result {
            result = x;
        }
    }
    result
}

/// the SAMPLE variance: divide by n - 1
pub fn variance(values: &[f64]) -> f64 {
    if values.is_empty() {
        return f64::NAN;
    }
    let mut sum_squared_deviations = 0.0;
    let avg = mean(values);
    for &x in values {
        sum_squared_deviations += (x - avg) * (x - avg);
    }
    sum_squared_deviations / (values.len() as f64 - 1.0)
}

/// the square root of variance()
pub fn std_dev(values: &[f64]) -> f64 {
    variance(values).sqrt()
}
