#[cfg(test)]
use std::hash::Hash;

pub mod bubble_sort;
pub mod bead_sort;

#[cfg(test)]
pub fn is_sorted<T>(arr: &[T]) -> bool
    where
        T: PartialOrd,
{
    arr.windows(2).all(|w| w[0] <= w[1])
}

#[cfg(test)]
pub fn have_same_elements<T>(a: &[T], b: &[T]) -> bool
    where
        T: PartialOrd + Eq + Hash,
{
    use std::collections::HashSet;

    match a.len() == b.len() {
        true => {
            let set_a: HashSet<&T> = a.iter().collect();
            let set_b: HashSet<&T> = b.iter().collect();
            set_a == set_b
        }
        false => false
    }
}