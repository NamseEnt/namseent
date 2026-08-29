pub fn manhattan_route_distance(
    route: &[(usize, usize)],
    left: usize,
    top: usize,
    fallback: usize,
) -> usize {
    route
        .iter()
        .map(|(x, y)| x.abs_diff(left) + y.abs_diff(top))
        .min()
        .unwrap_or(fallback)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_row_distance_is_not_collapsed_to_zero() {
        assert_eq!(manhattan_route_distance(&[(0, 0), (10, 0)], 5, 0, 99), 5);
    }
}
