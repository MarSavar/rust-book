// Given a list of integers, use a vector and return the mean (the average value),
// median (when sorted, the value in the middle position),
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.

use std::collections::HashMap;

#[derive(Debug)]
pub struct NumVector {
    pub numbers: Vec<i32>
}

impl NumVector {
    pub fn mean(&self) -> f32 {
        let sum: f32 = self.numbers.iter().map(|&n| n as f32).sum();
        sum / self.numbers.len() as f32
    }

    pub fn median(&self) -> i32 {
        let mut sorted = self.numbers.to_vec();
        sorted.sort();
        sorted[sorted.len() / 2]
    }

    pub fn mode(&self) -> &i32 {
        let mut map = HashMap::new();
        for i in &self.numbers {
            *map.entry(i).or_insert(0) += 1;
        }

        map
            .into_iter()
            .max_by_key(|&(_k, v)| v)
            .map(|(k, _v)| k).expect("Empty vector")
    }
}
