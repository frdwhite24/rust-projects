use std::collections::HashMap;

#[derive(Debug)]
enum Median {
    Value(i32),
    Average(f64),
}

#[derive(Debug)]
enum Mode {
    Multiple(Vec<i32>),
    Single(i32),
}

struct Comparison {
    is_larger: bool,
    is_equal: bool,
}

fn main() {
    print_results(vec![1, 2, 3, 4, 5, 6, 21, 56, 77, 77, 4, 6]);
    print_results(vec![1, -1, 1, -1, 2]);
    print_results(vec![0, 0, 0, 0]);
    print_results(vec![]);
}

fn print_results(data: Vec<i32>) {
    println!("our data: {:?}", data);
    println!("mean: {:?}", get_mean(&data));
    println!("mode: {:?}", get_mode(&data));
    println!("median: {:?}\n\n", get_median(&data));
}

fn get_mean(data: &Vec<i32>) -> Option<f64> {
    if data.len() == 0 {
        return None;
    }

    let mut total = 0 as f64;
    for value in data {
        total += f64::from(*value);
    }
    Some(total / data.len() as f64)
}

fn get_entry_counts(data: &Vec<i32>) -> HashMap<&i32, i32> {
    let mut value_counts = HashMap::new();
    for value in data {
        let count = value_counts.entry(value).or_insert(0);
        *count += 1;
    }
    value_counts
}

fn compare_to_modes(
    modes: &Vec<i32>,
    value_counts: &HashMap<&i32, i32>,
    count: &i32,
) -> Comparison {
    let mut is_larger = false;
    let mut is_equal = false;

    for mode in modes {
        match value_counts.get(mode) {
            Some(mode_count) => {
                if count > mode_count {
                    is_larger = true;
                } else if count == mode_count {
                    is_equal = true;
                }
            }
            _ => (),
        }
    }

    Comparison {
        is_equal,
        is_larger,
    }
}

fn get_mode(data: &Vec<i32>) -> Option<Mode> {
    if data.len() == 0 {
        return None;
    };

    let value_counts = get_entry_counts(&data);

    // Extract mode or modes from count HashMap. Mode is value or values that
    // occur(s) the most number of times or None if all occur equal number of times
    let mut modes = vec![];
    for (value, count) in &value_counts {
        // Handle first case where no other values to compare
        if modes.len() == 0 {
            modes.push(**value);
            continue;
        }

        let Comparison {
            is_larger,
            is_equal,
        } = compare_to_modes(&modes, &value_counts, count);

        // Replace all existing modes with new larger mode, or append equally
        // occuring mode
        if is_larger == true {
            modes.clear();
            modes.push(**value);
        } else if is_equal == true {
            modes.push(**value);
        }
    }

    // This might be able to be handled better, case where all values occur
    // equally and therefore there is no mode for the vector
    if modes.len() == data.len() {
        return None;
    }

    // Return the correct type
    if modes.len() > 1 {
        return Some(Mode::Multiple(modes));
    } else if modes.len() == 1 {
        return Some(Mode::Single(modes[0]));
    } else {
        return None;
    }
}

fn get_median(data: &Vec<i32>) -> Option<Median> {
    if data.len() == 0 {
        return None;
    };

    let mut sorted = data.clone();
    sorted.sort();

    if data.len() % 2 == 0 {
        let middle_left = match sorted.get(data.len() / 2) {
            None => None,
            Some(val) => Some(f64::from(*val)),
        };

        let middle_right = match sorted.get(data.len() / 2) {
            None => None,
            Some(val) => Some(f64::from(*val)),
        };

        match middle_left {
            None => None,
            Some(left) => match middle_right {
                None => None,
                Some(right) => Some(Median::Average((left + right) / 2f64)),
            },
        }
    } else {
        match sorted.get(data.len() / 2) {
            None => None,
            Some(value) => Some(Median::Value(*value)),
        }
    }
}
