use namui::*;
use std::collections::VecDeque;

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
    if end_xy == start_xy {
        return Some(vec![start_xy]);
    }

    if blockers.contains(&start_xy) || blockers.contains(&end_xy) {
        return None;
    }

    // This algorithm is BFS, written by consuming that,
    // 1. the map is a grid,
    // 2. weight is constant,
    // 3. moving up and right takes 2 steps but moving diagonally takes 1 steps.
    // If you met any changes, make sure to adjust the algorithm.

    let mut map = Map::new(wh, blockers);

    let mut queue = VecDeque::new();

    queue.push_back(start_xy);
    map.set_visit(start_xy, start_xy);

    while let Some(from_xy) = queue.pop_front() {
        for xy in neighbor(from_xy) {
            if map.cannot_visit(xy, from_xy) {
                continue;
            }
            queue.push_back(xy);
            map.set_visit(xy, from_xy);

            if xy == end_xy {
                return Some(map.gather_route(xy));
            }
        }
    }

    None
}

fn neighbor(last_xy: Xy<usize>) -> impl Iterator<Item = Xy<usize>> {
    DX_DY.iter().filter_map(move |(dx, dy)| {
        let x = last_xy.x as isize + dx;
        let y = last_xy.y as isize + dy;
        if x >= 0 && y >= 0 {
            Some(Xy::new(x as usize, y as usize))
        } else {
            None
        }
    })
}

#[derive(Clone, Copy, Debug, PartialEq, State)]
enum Block {
    Empty,
    Blocker,
    Visited { from_xy: Xy<usize> },
}

struct Map {
    wh: Wh<usize>,
    blocks: Vec<Block>,
}

impl Map {
    fn new(wh: Wh<usize>, blockers: &[Xy<usize>]) -> Self {
        let blocks = vec![Block::Empty; wh.width * wh.height];
        let mut this = Self { wh, blocks };
        for &blocker in blockers {
            *this.block_mut(blocker) = Block::Blocker;
        }
        this
    }

    fn block(&self, xy: Xy<usize>) -> Block {
        self.blocks[xy.y * self.wh.width + xy.x]
    }

    fn block_mut(&mut self, xy: Xy<usize>) -> &mut Block {
        &mut self.blocks[xy.y * self.wh.width + xy.x]
    }

    fn set_visit(&mut self, xy: Xy<usize>, from_xy: Xy<usize>) {
        *self.block_mut(xy) = Block::Visited { from_xy };
    }

    fn cannot_visit(&self, xy: Xy<usize>, from_xy: Xy<usize>) -> bool {
        if self.is_outside(xy) {
            return true;
        }
        match self.block(xy) {
            Block::Blocker | Block::Visited { .. } => true,
            Block::Empty => !is_orthogonal(from_xy, xy) && self.blocked_on_diagonal(from_xy, xy),
        }
    }

    fn is_outside(&self, xy: Xy<usize>) -> bool {
        xy.x >= self.wh.width || xy.y >= self.wh.height
    }

    fn gather_route(&self, end_xy: Xy<usize>) -> Vec<Xy<usize>> {
        let mut route = vec![end_xy];
        let mut xy = end_xy;
        loop {
            match self.block(xy) {
                Block::Visited { from_xy } => {
                    if xy == from_xy {
                        break;
                    }
                    route.push(from_xy);
                    xy = from_xy;
                }
                _ => unreachable!(),
            }
        }
        route.reverse();
        route
    }

    fn blocked_on_diagonal(&self, from_xy: Xy<usize>, xy: Xy<usize>) -> bool {
        let a_xy = Xy::new(from_xy.x, xy.y);
        let b_xy = Xy::new(xy.x, from_xy.y);
        self.block(a_xy) == Block::Blocker && self.block(b_xy) == Block::Blocker
    }
}

fn is_orthogonal(from_xy: Xy<usize>, xy: Xy<usize>) -> bool {
    from_xy.x == xy.x || from_xy.y == xy.y
}

/// Orthogonal first, then diagonal.
const DX_DY: [(isize, isize); 8] = [
    (0, -1),
    (-1, 0),
    (1, 0),
    (0, 1),
    (1, 1),
    (-1, -1),
    (1, -1),
    (-1, 1),
];

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
