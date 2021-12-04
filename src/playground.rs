#[cfg(test)]
mod tests {
    use log::debug;
    use ndarray::Array2;
    use test_log::test;

    #[test]
    fn some_test() {
        let array = Array2::from_shape_vec((2, 2), vec![1, 2, 3, 4]).unwrap();
        for e in array.diag() {
            debug!("{}", e);
        }
        for e in array.view().reversed_axes().diag() {
            debug!("{}", e);
        }
    }
}
