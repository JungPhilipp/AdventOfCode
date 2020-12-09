use std::collections::HashSet;

pub fn two_sum<'a, T>(numbers: &'a Vec<T>, sum: &'a T) -> Vec<[T; 2]>
where
    T: num::Integer + std::hash::Hash + std::clone::Clone,
    &'a T: std::ops::Sub<Output = T>,
{
    let mut valid_numbers = HashSet::new();
    let mut lookup = HashSet::new();

    for number in numbers {
        if lookup.contains(&(sum - number)) {
            valid_numbers.insert(number);
        } else {
            lookup.insert(number);
        }
    }
    valid_numbers
        .into_iter()
        .map(|x| {
            let c = sum - x;
            if *x < c {
                [(*x).clone(), c]
            } else {
                [c, (*x).clone()]
            }
        })
        .collect()
}

pub fn three_sum(numbers: &Vec<i32>, sum: i32) -> Vec<[i32; 3]> {
    let mut valid_numbers = HashSet::new();
    for number in numbers {
        let search = sum - number;
        let found = two_sum(numbers, &search);
        for pair in found {
            valid_numbers.insert([*number, pair[0], pair[1]]);
        }
    }
    valid_numbers.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert!(two_sum(&vec![], &0).is_empty());
        assert_eq!(two_sum(&vec![10, 20, 30], &30), vec![[10, 20]]);
    }
}
