#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn contains<T: PartialEq + Ord + Clone>(list1: &[T], list2: &[T]) -> bool {
    for window in list1.windows(list2.len()) {
        if window == list2 {
            return true;
        }
    }

    false
}

pub fn sublist<T: PartialEq + Ord + Clone>(list1: &[T], list2: &[T]) -> Comparison {
    let first_len = list1.len();
    let second_len = list2.len();

    if first_len == 0 && second_len == 0 {
        return Comparison::Equal;
    }

    if list1 == list2 {
        return Comparison::Equal;
    }

    if first_len > 0 && second_len == 0 {
        return Comparison::Superlist;
    }

    if first_len == 0 && second_len > 0 {
        return Comparison::Sublist;
    }

    if contains(list1, list2) {
        return Comparison::Superlist;
    }

    if contains(list2, list1) {
        return Comparison::Sublist;
    }

    Comparison::Unequal
}
