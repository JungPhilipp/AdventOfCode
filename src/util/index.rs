pub fn expand(flat_index: i32, dimensions: &(usize, usize)) -> (i32, i32) {
    (
        flat_index % dimensions.0 as i32,
        flat_index / dimensions.0 as i32,
    )
}

pub fn flatten(pos: (i32, i32), dimensions: &(usize, usize)) -> Option<usize> {
    if (0..dimensions.0 as i32).contains(&pos.0) && (0..dimensions.1 as i32).contains(&pos.1) {
        Some((pos.1 * dimensions.0 as i32 + pos.0) as usize)
    } else {
        None
    }
}
