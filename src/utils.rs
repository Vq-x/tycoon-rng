use num::{Num, NumCast, ToPrimitive};
use rand::{seq::SliceRandom, thread_rng, Rng};

pub fn human_readable<T>(mut value: T) -> String
where
    T: Num + NumCast + ToPrimitive + Copy,
{
    static SUFFIXES: [&str; 25] = [
        "", "K", "M", "B", "T", "qd", "Qn", "sx", "Sp", "Oc", "No", "De", "Ud", "DD", "tdD", "qdD",
        "QnD", "sxD", "SpD", "OcD", "NvD", "Vgn", "UVg", "DVg", "TVg",
    ];
    let mut i = 0;

    while value.to_f64().unwrap() >= 1000.0 && i < SUFFIXES.len() - 1 {
        value = NumCast::from(value.to_f64().unwrap() / 1000.0).unwrap();
        i += 1;
    }

    format!("{:.1}{}", value.to_f64().unwrap(), SUFFIXES[i])
}
pub fn mix_vectors_evenly<T: Clone>(vec1: &mut Vec<T>, vec2: Vec<T>) -> Vec<T> {
    let mut mixed = Vec::with_capacity(vec1.len() + vec2.len());

    let (larger_vec, smaller_vec) = if vec1.len() >= vec2.len() {
        (vec1.as_slice(), vec2)
    } else {
        (vec2.as_slice(), vec1.clone())
    };

    let larger_len = larger_vec.len();
    let smaller_len = smaller_vec.len();
    let step = larger_len as f64 / smaller_len as f64;

    let mut smaller_index = 0;
    for (i, larger_item) in larger_vec.iter().enumerate() {
        mixed.push(larger_item.clone());

        let expected_position = ((i + 1) as f64 / step).round() as usize;
        if expected_position > smaller_index && smaller_index < smaller_len {
            mixed.push(smaller_vec[smaller_index].clone());
            smaller_index += 1;
        }
    }

    mixed
}

pub fn apply_with_chance<T, F>(item: &mut T, chance: f64, mut func: F) -> bool
where
    F: FnMut(&mut T),
{
    let mut rng = rand::thread_rng();

    // Apply the function with the specified chance
    if rng.gen_bool(chance / 100.0) {
        func(item);
        true
    } else {
        false
    }
}
