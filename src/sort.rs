// mergesort helper
pub fn merge(left: &mut Vec<i32>, right: &mut Vec<i32>) -> Vec<i32> {
    let (mut l, mut r) = (0, 0);
    let mut result_vector = Vec::new();

    while l < left.len() && r < right.len() {
        if left[l] < right[r] {
            result_vector.push(left[l]);
            l += 1;
        } else {
            result_vector.push(right[r]);
            r += 1;
        }
    }

    while l < left.len() {
        result_vector.push(left[l]);
        l += 1;
    }

    while r < right.len() {
        result_vector.push(right[r]);
        r += 1;
    }

    return result_vector;
}

// merge algorithm
pub fn merge_sort(input_vector: &mut Vec<i32>) {
    if input_vector.len() <= 1 {
        return;
    }

    let mid = input_vector.len() / 2;
    let left = &mut input_vector[..mid].to_vec();
    let right = &mut input_vector[mid..].to_vec();

    merge_sort(left);
    merge_sort(right);

    *input_vector = merge(left, right);
}

// quicksort alg
pub fn quick_sort(input_vector: &mut [i32]) {
    if input_vector.len() <= 1 {
        return;
    }
    let pivot = input_vector[input_vector.len() - 1];

    let mut i = 0;
    for j in 0..input_vector.len() - 1 {
        if input_vector[j] < pivot {
            input_vector.swap(i, j);
            i += 1;
        }
    }
    input_vector.swap(i, input_vector.len() - 1);

    quick_sort(&mut input_vector[..i]);
    quick_sort(&mut input_vector[i + 1..]);
}
