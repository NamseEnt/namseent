use namui_type::*;
use std::collections::VecDeque;

/// # Parameters
/// * `blocks`: blocks[x][y] = true: NotEmpty, false: Empty
pub fn bfs(
    blocks: Vec<Vec<bool>>,
    start_xy: Xy<usize>,
    end_xy: Xy<usize>,
) -> Option<Vec<Xy<usize>>> {
    if start_xy == end_xy {
        return Some(vec![start_xy]);
    }

    let mut queue = VecDeque::new();
    queue.push_back(start_xy);

    let width = blocks[0].len();
    let height = blocks.len();

    let mut visited_from = vec![vec![None; height]; width];
    visited_from[start_xy.x][start_xy.y] = Some(start_xy);

    while let Some(xy) = queue.pop_front() {
        if xy == end_xy {
            let mut path = Vec::new();
            let mut pos = xy;

            while pos != start_xy {
                path.push(pos);
                pos = visited_from[pos.x][pos.y].unwrap();
            }
            path.push(start_xy);
            path.reverse();

            return Some(path);
        }

        const DIRECTIONS: [(isize, isize); 4] = [(0isize, -1isize), (1, 0), (0, 1), (-1, 0)];

        for (dx, dy) in DIRECTIONS {
            let next_x = if (dx < 0 && xy.x == 0) || (dx > 0 && xy.x + 1 >= width) {
                continue;
            } else {
                (xy.x as isize + dx) as usize
            };

            let next_y = if (dy < 0 && xy.y == 0) || (dy > 0 && xy.y + 1 >= height) {
                continue; // 아래로 갈 수 없음 (경계)
            } else {
                (xy.y as isize + dy) as usize
            };

            if visited_from[next_x][next_y].is_some() || blocks[next_x][next_y] {
                continue;
            }

            visited_from[next_x][next_y] = Some(xy);

            queue.push_back(Xy {
                x: next_x,
                y: next_y,
            });
        }
    }

    None
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_bfs() {
//         let map = vec![vec![true, true], vec![false, false]];

//         bfs(map.iter().map(|row| {
//             row.iter().map(|&cell| {
//                 if cell {
//                     BfsMapBlock::Empty
//                 } else {
//                     BfsMapBlock::NotEmpty
//                 }
//             })
//         }));
//     }
// }
