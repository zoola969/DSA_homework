use std::cmp::Ordering;

struct Fib {
    current: usize,
    next: usize,
}

impl Iterator for Fib {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        self.current = self.next;
        self.next += current;
        Some(current)
    }
}

pub fn fib_search<T: Ord>(slice: &[T], value: &T) -> (Option<usize>, usize) {
    // Return the index of the element if it is presented in the slice and position where it can be added to.
    if slice.is_empty() {
        return (None, 0);
    }

    let fib = Fib {
        current: 0,
        next: 1,
    };
    let mut searchin_idx: usize;
    let last_idx = slice.len() - 1;
    let mut prev_idx: usize = 0;

    for index in fib {
        searchin_idx = index.min(last_idx);

        match slice[searchin_idx].cmp(value) {
            // todo make it find the first element
            Ordering::Equal => {
                return (Some(searchin_idx), searchin_idx + 1);
            }
            Ordering::Greater => {
                let (res, index_to_insert) = fib_search(&slice[prev_idx..searchin_idx], value);
                match res {
                    Some(offset) => {
                        return (Some(prev_idx + offset), prev_idx + index_to_insert);
                    }
                    None => {
                        return (None, prev_idx + index_to_insert);
                    }
                };
            }
            Ordering::Less => {
                prev_idx = searchin_idx;
            }
        };

        if searchin_idx == last_idx {
            break;
        }
    }

    (None, slice.len())
}

pub fn insert<T: Ord>(vec: &mut Vec<T>, value: T) -> usize {
    let (_, idx) = fib_search(vec, &value);
    vec.insert(idx, value);
    idx
}

pub fn delete<T: Ord>(vec: &mut Vec<T>, value: &T) -> Option<usize> {
    if let (Some(idx), _) = fib_search(vec, value) {
        vec.remove(idx);
        return Some(idx);
    }
    None
}
