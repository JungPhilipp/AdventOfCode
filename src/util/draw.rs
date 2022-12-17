#![allow(unused)]
pub fn grid_to_string(points: &[(i64, i64)]) -> String {
    let _min = {
        (
            *points.iter().map(|(x, _)| x).min().unwrap(),
            *points.iter().map(|(_, y)| y).min().unwrap(),
        )
    };
    let max = {
        (
            *points.iter().map(|(x, _)| x).max().unwrap(),
            *points.iter().map(|(_, y)| y).max().unwrap(),
        )
    };

    //let hash_points: HashSet<(i64, i64)> = points
    //    .iter()
    //    .map(|point| (point.0 - min.0, point.1 - min.1))
    //    .collect();
    let ref_points = &points;
    (0..=max.1)
        .rev()
        .flat_map(move |y| {
            (0..=max.0)
                .map(move |x| {
                    let point = (x, y);
                    if ref_points.contains(&point) {
                        '#'
                    } else {
                        '.'
                    }
                })
                .chain(['\n'].into_iter())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn draw_single_point() {
        assert_eq!(grid_to_string(&[(1, 1)]), ".#\n..\n");
    }

    #[test]
    fn draw_points() {
        assert_eq!(grid_to_string(&[(0, 0), (1, 1)]), ".#\n#.\n");
        assert_eq!(grid_to_string(&[(0, 0), (1, 0)]), "##\n");
        assert_eq!(grid_to_string(&[(0, 0), (0, 1)]), "#\n#\n");
    }
}
