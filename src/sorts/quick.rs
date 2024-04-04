pub fn sort_queue<T>(
    arr: &mut [T],
) -> Vec<Option<(usize, usize)>>
where
    T: std::cmp::PartialOrd + std::fmt::Debug,
{
    let mid = (arr.len() / 2).saturating_sub(1);
    let mut queue = vec![];

    quick_sort(arr, 0, arr.len(), &mut queue);
    let mut queue = queue.into_iter().map(|v| Some(v)).collect::<Vec<Option<(usize, usize)>>>();
    queue.push(None);
    queue
}

fn quick_sort<T>(arr: &mut [T], init: usize, end: usize, queue: &mut Vec<(usize, usize)>) 
where T: std::cmp::PartialOrd + std::fmt::Debug,
{
    if init < end {
        let p = partition_end(arr, init, end, queue);
        quick_sort(arr, init, p-1, queue);
        quick_sort(arr, p, end, queue);
    }
}

fn partition_end<T>(arr: &mut [T], init: usize, end: usize, queue: &mut Vec<(usize, usize)>) -> usize
where T: std::cmp::PartialOrd + std::fmt::Debug,
{
    // using end value as pivot
    let pivot = end-1;
    let mut j = init;
    for i in init..end-1 {
        if arr[i] < arr[pivot] {
            queue.push((i, j));
            arr.swap(i, j);
            j+=1;
        }
    }
    queue.push((j, pivot));
    arr.swap(j, pivot);
    j + 1
}

#[cfg(test)]
#[path = "../_tests/sort_quick.rs"]
mod test;
