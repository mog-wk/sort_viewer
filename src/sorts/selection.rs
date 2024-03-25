pub fn sort<T>(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    arr: &mut [T],
    ) -> Option<(usize, usize)> 
where 
    T: Clone + std::cmp::PartialOrd,
{
    for i in 0..arr.len() - 1 {
        for j in i + 1..arr.len() {
            if arr[i] > arr[j] {
                arr.swap(i, j);
                return Some((i, j));
            }
        }
    }
    None
}
