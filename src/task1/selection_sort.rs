pub fn selection_sort<T: Ord>(slice: &mut [T]) {
    let mut min_ptr = 0;

    (0..(slice.len() - 1)).for_each(|unsorted_elem_ptr| {
        min_ptr = unsorted_elem_ptr;
        (unsorted_elem_ptr..slice.len()).for_each(|i| {
            slice[i].cmp(&slice[min_ptr]).is_lt().then(|| min_ptr = i);
        });
        min_ptr
            .cmp(&unsorted_elem_ptr)
            .is_ne()
            .then(|| slice.swap(unsorted_elem_ptr, min_ptr));
    });
}
