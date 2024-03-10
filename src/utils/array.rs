
pub fn get_limits(arr: &[u32]) -> (u32, u32) {
    let (mut min, mut max): (u32, u32) = (std::u32::MAX, std::u32::MIN);
    for i in arr.iter() {
        if i > &max {
            max = i.clone();
        }
        if i < &min {
            min = i.clone();
        }
    }
    (min, max)
}

#[cfg(test)]
#[path = "../_tests/utils_array.rs"]
mod test;
