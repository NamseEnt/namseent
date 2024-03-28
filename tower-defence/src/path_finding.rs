use namui::*;

pub fn find_path(
    start_xy: Xy<usize>,
    end_xy: Xy<usize>,
    map: &[Vec<bool>],
    width: usize,
    height: usize,
) -> Option<Vec<Xy<usize>>> {
    let mut from_map: Vec<Vec<Option<Xy<usize>>>> = vec![vec![None; width]; height];
    from_map[start_xy.y][start_xy.x] = Some(start_xy);

    let mut next_steps: Vec<Xy<usize>> = vec![start_xy];
    let mut current_steps: Vec<Xy<usize>> = vec![];

    'outer: loop {
        std::mem::swap(&mut next_steps, &mut current_steps);
        next_steps.clear();
        if current_steps.is_empty() {
            return None;
        }

        enum Direction {
            Up,
            Right,
            Down,
            Left,
        }

        for xy in current_steps.iter().cloned() {
            for direction in [
                Direction::Up,
                Direction::Right,
                Direction::Down,
                Direction::Left,
            ] {
                if (xy.x == 0 && matches!(direction, Direction::Left))
                    || (xy.y == 0 && matches!(direction, Direction::Up))
                {
                    continue;
                }

                let next_xy = {
                    let mut next_xy = xy;

                    match direction {
                        Direction::Up => next_xy.y -= 1,
                        Direction::Right => next_xy.x += 1,
                        Direction::Down => next_xy.y += 1,
                        Direction::Left => next_xy.x -= 1,
                    }

                    next_xy
                };

                if next_xy.x >= width || next_xy.y >= height {
                    continue;
                }

                if !map[next_xy.y][next_xy.x] || from_map[next_xy.y][next_xy.x].is_some() {
                    continue;
                }

                from_map[next_xy.y][next_xy.x] = Some(xy);

                if next_xy == end_xy {
                    break 'outer;
                }

                next_steps.push(next_xy);
            }
        }
    }

    let mut path = vec![end_xy];
    let mut current_xy = end_xy;
    while current_xy != start_xy {
        current_xy = from_map[current_xy.y][current_xy.x].unwrap();
        path.push(current_xy);
    }

    path.reverse();

    Some(path)
}
