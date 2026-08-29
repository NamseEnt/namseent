use namui::*;

/// # Diagonal movement
///
/// Diagonal movement is blocked if both sides are blocked.
///
/// ```text
/// f: from, t: to, b: blocker, 0: empty
///
/// // blocked
/// f b
/// b t
///
/// // not blocked
/// f 0
/// b t
///
/// f b
/// 0 t
///
/// f 0
/// 0 t
pub fn find_shortest_route(
    wh: Wh<usize>,
    start_xy: Xy<usize>,
    end_xy: Xy<usize>,
    blockers: &[Xy<usize>],
) -> Option<Vec<Xy<usize>>> {
    let blockers = blockers.iter().map(|xy| [xy.x, xy.y]).collect::<Vec<_>>();
    let route = td_core::find_shortest_route(
        [wh.width, wh.height],
        [start_xy.x, start_xy.y],
        [end_xy.x, end_xy.y],
        &blockers,
    )?;
    Some(route.into_iter().map(|[x, y]| Xy::new(x, y)).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_blockers() {
        let wh = Wh::new(5, 5);
        let start = Xy::new(0, 0);
        let end = Xy::new(4, 4);
        let blockers = [];
        let route = find_shortest_route(wh, start, end, &blockers).unwrap();
        assert_eq!(
            route,
            vec![start, Xy::new(1, 1), Xy::new(2, 2), Xy::new(3, 3), end]
        );
    }

    #[test]
    fn test_with_blockers() {
        let wh = Wh::new(5, 5);
        let start = Xy::new(0, 0);
        let end = Xy::new(4, 4);
        let blockers = vec![Xy::new(1, 1), Xy::new(2, 2), Xy::new(3, 3)];
        let route = find_shortest_route(wh, start, end, &blockers).unwrap();
        assert_eq!(
            route,
            vec![
                start,
                Xy::new(1, 0),
                Xy::new(2, 1),
                Xy::new(3, 2),
                Xy::new(4, 3),
                end
            ]
        );
    }

    #[test]
    fn test_no_possible_route() {
        let wh = Wh::new(5, 5);
        let start = Xy::new(0, 0);
        let end = Xy::new(4, 4);
        let blockers = vec![
            Xy::new(1, 0),
            Xy::new(1, 1),
            Xy::new(0, 1),
            Xy::new(3, 3),
            Xy::new(4, 3),
            Xy::new(3, 4),
        ];
        let route = find_shortest_route(wh, start, end, &blockers);
        assert!(route.is_none());
    }

    #[test]
    fn test_start_is_end() {
        let wh = Wh::new(5, 5);
        let start = Xy::new(2, 2);
        let end = Xy::new(2, 2);
        let blockers = vec![];
        let route = find_shortest_route(wh, start, end, &blockers).unwrap();
        assert_eq!(route, vec![start]);
    }

    #[test]
    fn test_blocked_diagonal() {
        let wh = Wh::new(5, 5);
        let start = Xy::new(0, 0);
        let end = Xy::new(2, 2);
        let blockers = vec![Xy::new(1, 0), Xy::new(0, 1)];
        let route = find_shortest_route(wh, start, end, &blockers);
        assert!(route.is_none());
    }

    #[test]
    fn test_complex_blockers() {
        let wh = Wh::new(7, 7);
        let start = Xy::new(0, 0);
        let end = Xy::new(6, 6);
        let blockers = vec![
            Xy::new(1, 1),
            Xy::new(2, 2),
            Xy::new(3, 3),
            Xy::new(4, 4),
            Xy::new(5, 5),
            Xy::new(1, 2),
            Xy::new(2, 3),
            Xy::new(3, 4),
            Xy::new(4, 5),
            Xy::new(5, 6),
        ];
        let route = find_shortest_route(wh, start, end, &blockers).unwrap();
        assert_eq!(
            route,
            vec![
                start,
                Xy::new(1, 0),
                Xy::new(2, 1),
                Xy::new(3, 2),
                Xy::new(4, 3),
                Xy::new(5, 4),
                Xy::new(6, 5),
                end
            ]
        );
    }

    #[test]
    fn test_edge_start_end() {
        let wh = Wh::new(5, 5);
        let start = Xy::new(0, 4);
        let end = Xy::new(4, 0);
        let blockers = vec![Xy::new(2, 2)];
        let route = find_shortest_route(wh, start, end, &blockers).unwrap();
        assert_eq!(
            route,
            vec![
                start,
                Xy::new(0, 3),
                Xy::new(1, 2),
                Xy::new(2, 1),
                Xy::new(3, 1),
                end
            ]
        );
    }

    #[test]
    fn test_blockers_surrounding_start() {
        let wh = Wh::new(5, 5);
        let start = Xy::new(2, 2);
        let end = Xy::new(4, 4);
        let blockers = vec![Xy::new(1, 2), Xy::new(2, 1), Xy::new(3, 2), Xy::new(2, 3)];
        let route = find_shortest_route(wh, start, end, &blockers);
        assert!(route.is_none());
    }

    #[test]
    fn test_blockers_surrounding_end() {
        let wh = Wh::new(5, 5);
        let start = Xy::new(0, 0);
        let end = Xy::new(2, 2);
        let blockers = vec![Xy::new(1, 2), Xy::new(2, 1), Xy::new(3, 2), Xy::new(2, 3)];
        let route = find_shortest_route(wh, start, end, &blockers);
        assert!(route.is_none());
    }

    #[test]
    fn test_large_grid() {
        let wh = Wh::new(100, 100);
        let start = Xy::new(0, 0);
        let end = Xy::new(99, 99);
        let blockers = vec![
            Xy::new(50, 50),
            Xy::new(51, 51),
            Xy::new(52, 52),
            Xy::new(53, 53),
        ];
        let route = find_shortest_route(wh, start, end, &blockers).unwrap();
        assert_eq!(route.len(), 101);
    }

    #[test]
    fn test_inner_start_end_no_blockers() {
        let wh = Wh::new(5, 5);
        let start = Xy::new(2, 2);
        let end = Xy::new(3, 3);
        let blockers = vec![];
        let route = find_shortest_route(wh, start, end, &blockers).unwrap();
        assert_eq!(route, vec![start, end]);
    }

    #[test]
    fn test_inner_start_end_with_blockers() {
        let wh = Wh::new(5, 5);
        let start = Xy::new(2, 2);
        let end = Xy::new(3, 3);
        let blockers = vec![Xy::new(2, 3)];
        let route = find_shortest_route(wh, start, end, &blockers).unwrap();
        assert_eq!(route, vec![start, end]);
    }

    #[test]
    fn test_inner_no_possible_route() {
        let wh = Wh::new(5, 5);
        let start = Xy::new(2, 2);
        let end = Xy::new(3, 3);
        let blockers = vec![Xy::new(2, 3), Xy::new(3, 2), Xy::new(3, 3)];
        let route = find_shortest_route(wh, start, end, &blockers);
        assert!(route.is_none());
    }

    #[test]
    fn test_inner_blocked_diagonal_outer_detour() {
        let wh = Wh::new(5, 5);
        let start = Xy::new(2, 2);
        let end = Xy::new(4, 4);
        let blockers = vec![Xy::new(3, 2), Xy::new(2, 3)];
        let route = find_shortest_route(wh, start, end, &blockers).unwrap();
        assert_eq!(
            route,
            vec![start, Xy::new(3, 1), Xy::new(4, 2), Xy::new(4, 3), end]
        );
    }

    #[test]
    fn test_inner_complex_blockers() {
        let wh = Wh::new(7, 7);
        let start = Xy::new(2, 2);
        let end = Xy::new(5, 5);
        let blockers = vec![Xy::new(3, 3), Xy::new(4, 4), Xy::new(3, 4), Xy::new(4, 5)];

        // map
        // start: s, end: e, blocker: #, empty: .
        // y↓, x→
        // . 0 1 2 3 4 5 6
        // 0 . . . . . . .
        // 1 . . . . . . .
        // 2 . . s . . . .
        // 3 . . . # . . .
        // 4 . . . # # . .
        // 5 . . . . # e .
        // 6 . . . . . . .

        let route = find_shortest_route(wh, start, end, &blockers).unwrap();
        assert_eq!(
            route,
            vec![start, Xy::new(3, 2), Xy::new(4, 3), Xy::new(5, 4), end]
        );
    }
}
