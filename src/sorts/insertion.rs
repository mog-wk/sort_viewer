pub fn sort_path<T>(arr: &mut [T]) -> Vec<(usize, usize)>
where
    T: Clone + std::cmp::PartialOrd,
{
    let sz = arr.len();
    let mut path = Vec::new();
    for i in 1..sz {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            j -= 1;
            arr.swap(j + 1, j);
            path.push((j + 1, j));
        }
    }
    path
}

pub fn sort<T>(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    arr: &mut [T],
) -> Option<(usize, usize)>
where
    T: Clone + std::cmp::PartialOrd,
{
    let sz = arr.len();

    for i in 1..sz {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            j -= 1;
            arr.swap(j + 1, j);
            return Some((j+1, j));
        }
    }
    None
}
