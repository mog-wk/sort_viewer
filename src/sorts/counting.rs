
fn sort(arr: &mut [i32], max: i32) {
    let max = max as usize;
    let mut count_arr = vec![0; max+1];

    for i in 0..arr.len() {
        let cur = arr[i];
        count_arr[cur as usize] += 1;
    }

    let mut i = 0;
    for j in 0..max+1 {
        for _ in 0..count_arr[j] {
            arr[i] = j as i32;
            i += 1;
        }
    }
}

#[cfg(test)]
#[path="../_tests/sort_couting.rs"]
mod tests;
