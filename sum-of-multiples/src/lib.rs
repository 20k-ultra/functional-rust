use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    // Create hash set to only add unique values in the end
    let mut unique_multiples = HashSet::new();
    // Populate hash set
    factors.iter().for_each(|factor| {
        if factor < &1 {
            return;
        }
        (1..(limit / factor) + 1)
            .filter(|x| x * factor < limit)
            .for_each(|x| {
                unique_multiples.insert(x * factor);
            })
    });
    // Sum hash set
    unique_multiples.iter().sum()
}