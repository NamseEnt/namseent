use namui_type::*;
use std::collections::VecDeque;

/// # Parameters
/// * `blocks`: `blocks[y][x] = true` for blocked, `false` for open.
///
/// # Returns
/// * `None` if no path found.
pub fn bfs(blocks: &[Vec<bool>], start_xy: Xy<usize>, end_xy: Xy<usize>) -> Option<Vec<Xy<usize>>> {
    let height = blocks.len();
    if height == 0 {
        return None;
    }
    let width = blocks[0].len();
    if width == 0 {
        return None;
    }
    let wh = Wh::new(width, height);

    let out_of_bounds =
        start_xy.y >= height || start_xy.x >= width || end_xy.y >= height || end_xy.x >= width;
    if out_of_bounds {
        return None;
    }

    let start_blocked = blocks[start_xy.y][start_xy.x];
    let end_blocked = blocks[end_xy.y][end_xy.x];

    if start_blocked || end_blocked {
        return None;
    }

    if start_xy == end_xy {
        return Some(vec![start_xy]);
    }

    let visit_map = search(blocks, wh, start_xy, end_xy)?;

    let mut path = vec![end_xy];
    loop {
        let last = path.last().unwrap();
        let next = visit_map[last.y][last.x].unwrap();
        path.push(next);
        if next == start_xy {
            break;
        }
    }
    path.reverse();
    Some(path)
}

fn search(
    blocks: &[Vec<bool>],
    wh: Wh<usize>,
    start_xy: Xy<usize>,
    end_xy: Xy<usize>,
) -> Option<Vec<Vec<Option<Xy<usize>>>>> {
    let mut visit_map: Vec<Vec<Option<Xy<usize>>>> = vec![vec![None; wh.width]; wh.height];
    visit_map[start_xy.y][start_xy.x] = Some(start_xy);

    let mut queue: VecDeque<Xy<usize>> = VecDeque::new();
    queue.push_back(start_xy);

    while let Some(xy) = queue.pop_front() {
        const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        for (dx, dy) in DIRECTIONS {
            let Some(next_x) = xy.x.checked_add_signed(dx) else {
                continue;
            };
            let Some(next_y) = xy.y.checked_add_signed(dy) else {
                continue;
            };

            if next_y < wh.height
                && next_x < wh.width
                && !blocks[next_y][next_x]
                && visit_map[next_y][next_x].is_none()
            {
                visit_map[next_y][next_x] = Some(xy);

                let next_xy = Xy::new(next_x, next_y);
                if next_xy == end_xy {
                    return Some(visit_map);
                }
                queue.push_back(next_xy);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_path_finding() {
        let blocks = vec![
            vec![false, false, false, false, false],
            vec![false, true, true, true, false],
            vec![false, false, false, false, false],
            vec![false, true, true, true, false],
            vec![false, false, false, false, false],
        ];

        let start_xy = Xy::new(0, 0);
        let end_xy = Xy::new(4, 4);

        let path = bfs(&blocks, start_xy, end_xy).unwrap();

        assert_eq!(path[0], start_xy);
        assert_eq!(path[path.len() - 1], end_xy);

        for i in 1..path.len() {
            let prev = path[i - 1];
            let current = path[i];

            assert!(
                (prev.x as isize - current.x as isize).abs()
                    + (prev.y as isize - current.y as isize).abs()
                    == 1,
                "Path points should be adjacent"
            );
        }
    }

    #[test]
    fn test_same_start_end() {
        let blocks = vec![vec![false, false], vec![false, false]];

        let xy = Xy::new(0, 0);

        let path = bfs(&blocks, xy, xy).unwrap();

        assert_eq!(path.len(), 1);
        assert_eq!(path[0], xy);
    }

    #[test]
    fn test_no_path() {
        let blocks = vec![
            vec![false, false, false],
            vec![false, true, false],
            vec![true, true, true],
        ];

        let start_xy = Xy::new(0, 0);
        let end_xy = Xy::new(2, 2);

        assert!(bfs(&blocks, start_xy, end_xy).is_none());
    }

    #[test]
    fn test_empty_grid() {
        let empty_blocks: Vec<Vec<bool>> = vec![];

        let start_xy = Xy::new(0, 0);
        let end_xy = Xy::new(1, 1);

        assert!(bfs(&empty_blocks, start_xy, end_xy).is_none());
    }

    #[test]
    fn test_out_of_bounds() {
        let blocks = vec![vec![false, false], vec![false, false]];

        let start_xy = Xy::new(0, 0);
        let end_xy = Xy::new(2, 2); // Out of bounds

        assert!(bfs(&blocks, start_xy, end_xy).is_none());
    }

    #[test]
    fn test_blocked_start_or_end() {
        let blocks = vec![vec![true, false], vec![false, false]];

        let start_xy = Xy::new(0, 0); // Start point is blocked
        let end_xy = Xy::new(1, 1);

        assert!(bfs(&blocks, start_xy, end_xy).is_none());

        let blocks = vec![vec![false, false], vec![false, true]];

        let start_xy = Xy::new(0, 0);
        let end_xy = Xy::new(1, 1); // End point is blocked

        assert!(bfs(&blocks, start_xy, end_xy).is_none());
    }

    #[test]
    fn test_simple_path() {
        let blocks = vec![
            vec![false, false, false],
            vec![false, true, false],
            vec![false, false, false],
        ];
        let start = Xy::new(0, 0);
        let end = Xy::new(2, 2);
        let expected = Some(vec![
            Xy::new(0, 0),
            Xy::new(0, 1),
            Xy::new(0, 2),
            Xy::new(1, 2),
            Xy::new(2, 2),
        ]);
        assert_eq!(bfs(&blocks, start, end), expected);
    }

    #[test]
    fn test_no_path_completely_blocked() {
        let blocks = vec![
            vec![false, true, false],
            vec![false, true, false],
            vec![false, true, false],
        ];
        let start = Xy::new(0, 0);
        let end = Xy::new(2, 2);
        assert_eq!(bfs(&blocks, start, end), None);
    }

    #[test]
    fn test_start_equals_end_open() {
        let blocks = vec![vec![false]];
        let start_end = Xy::new(0, 0);
        assert_eq!(bfs(&blocks, start_end, start_end), Some(vec![start_end]));
    }

    #[test]
    fn test_start_equals_end_blocked() {
        let blocks = vec![vec![true]];
        let start_end = Xy::new(0, 0);
        assert_eq!(bfs(&blocks, start_end, start_end), None);
    }

    #[test]
    fn test_start_is_blocked() {
        let blocks = vec![vec![true, false], vec![false, false]];
        let start = Xy::new(0, 0);
        let end = Xy::new(1, 1);
        assert_eq!(bfs(&blocks, start, end), None);
    }

    #[test]
    fn test_end_is_blocked() {
        let blocks = vec![vec![false, false], vec![false, true]];
        let start = Xy::new(0, 0);
        let end = Xy::new(1, 1);
        assert_eq!(bfs(&blocks, start, end), None);
    }

    #[test]
    fn test_start_out_of_bounds() {
        let blocks = vec![vec![false]];
        let start = Xy::new(1, 0); // Out of bounds
        let end = Xy::new(0, 0);
        assert_eq!(bfs(&blocks, start, end), None);
    }

    #[test]
    fn test_end_out_of_bounds() {
        let blocks = vec![vec![false]];
        let start = Xy::new(0, 0);
        let end = Xy::new(0, 1); // Out of bounds
        assert_eq!(bfs(&blocks, start, end), None);
    }

    #[test]
    fn test_grid_with_empty_row() {
        let blocks: Vec<Vec<bool>> = vec![vec![]];
        let start = Xy::new(0, 0);
        let end = Xy::new(0, 0);
        assert_eq!(bfs(&blocks, start, end), None);
    }

    #[test]
    fn test_complex_maze_path() {
        let blocks = vec![
            vec![false, false, true, false, false],
            vec![true, false, true, false, true],
            vec![false, false, false, false, false],
            vec![false, true, true, true, false],
            vec![false, false, false, false, false],
        ];
        let start = Xy::new(0, 0);
        let end = Xy::new(4, 4);
        let expected = Some(vec![
            Xy::new(0, 0),
            Xy::new(1, 0),
            Xy::new(1, 1),
            Xy::new(1, 2),
            Xy::new(2, 2),
            Xy::new(3, 2),
            Xy::new(4, 2),
            Xy::new(4, 3),
            Xy::new(4, 4),
        ]);
        assert_eq!(bfs(&blocks, start, end), expected);
    }
}
