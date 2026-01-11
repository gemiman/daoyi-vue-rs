use std::collections::HashSet;

pub fn intersection_distinct<T: Eq + std::hash::Hash + Clone>(a: &[T], b: &[T]) -> Vec<T> {
    let set_a: HashSet<&T> = a.iter().collect();
    b.iter()
        .filter(|item| set_a.contains(item))
        .collect::<HashSet<&T>>()
        .into_iter()
        .cloned()
        .collect()
}
