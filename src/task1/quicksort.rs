pub fn quicksort<T: Ord>(slice: &mut [T]) {
    if slice.len() <= 1 {
        return;
    }
    let mut i = 1;
    let mut j = slice.len() - 1;
    loop {
        while i < slice.len() && slice[i] <= slice[0] {
            i += 1;
        }
        while j > 0 && slice[j] > slice[0] {
            j -= 1;
        }
        if i >= j {
            break;
        }
        slice.swap(i, j);
        i += 1;
        j -= 1;
    }
    slice.swap(0, j);

    quicksort(&mut slice[0..j]);
    quicksort(&mut slice[j + 1..]);
}
