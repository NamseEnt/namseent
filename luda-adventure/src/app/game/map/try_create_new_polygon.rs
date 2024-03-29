use geo::{coord, Area, Coord, EuclideanDistance, Line, LineString, Polygon};
use namui::*;
use std::collections::HashMap;

pub(super) fn try_create_new_polygon(
    wall: &Vec<String>,
    visit_map: &mut Vec<Vec<bool>>,
    start_xy: Xy<usize>,
) -> Option<Polygon> {
    let lines = get_lines(wall, visit_map, start_xy);
    let lines = simplify_lines(lines);
    let line_strings = connect_lines_into_line_strings(lines);
    build_line_strings_into_polygon(line_strings)
}

fn build_line_strings_into_polygon(mut line_strings: Vec<LineString>) -> Option<Polygon> {
    line_strings.sort_by(|a, b| a.unsigned_area().total_cmp(&b.unsigned_area()));
    match line_strings.pop() {
        Some(widest_line_string) => Some(Polygon::new(widest_line_string, line_strings)),
        None => None,
    }
}

fn connect_lines_into_line_strings(lines: Vec<Line>) -> Vec<LineString> {
    let mut line_strings = Vec::new();
    let mut start_point_line_map = HashMap::new();
    for line in lines.into_iter() {
        let key = key_from_coord(line.start);
        start_point_line_map.insert(key, line);
    }

    while let Some(first_key) = start_point_line_map.keys().next().copied() {
        let mut merging_line = start_point_line_map.remove(&first_key).unwrap();
        let mut line_string = LineString::new(vec![merging_line.start]);

        loop {
            let next_line_key = key_from_coord(merging_line.end);
            if let Some(next_line) = start_point_line_map.remove(&next_line_key) {
                line_string.0.push(next_line.start);
                merging_line = next_line;
            } else {
                let start_point_of_line_string = line_string.coords().next().unwrap();
                let loop_complete = merging_line.end == *start_point_of_line_string;
                if loop_complete {
                    line_strings.push(line_string);
                }
                break;
            }
        }
    }

    line_strings
}

fn simplify_lines(lines: Vec<Line>) -> Vec<Line> {
    let mut simplified_lines = Vec::new();
    let mut start_point_line_map = HashMap::new();
    for line in lines.into_iter() {
        let key = key_from_coord(line.start);
        start_point_line_map.insert(key, line);
    }

    while let Some(first_key) = start_point_line_map.keys().next().copied() {
        let mut merging_line = start_point_line_map.remove(&first_key).unwrap();
        loop {
            let next_line_key = key_from_coord(merging_line.end);
            if let Some(next_line) = start_point_line_map.remove(&next_line_key) {
                if let Some(merged_line) = try_merge_line(merging_line, next_line) {
                    merging_line = merged_line;
                } else {
                    simplified_lines.push(merging_line);
                    merging_line = next_line;
                }
            } else {
                simplified_lines.push(merging_line);
                break;
            }
        }
    }

    simplified_lines
}
fn key_from_coord(coord: Coord<f64>) -> (u64, u64) {
    (coord.x.to_bits(), coord.y.to_bits())
}

fn try_merge_line(a: Line, b: Line) -> Option<Line> {
    if !same_direction(a, b) {
        None
    } else if a.end == b.start {
        Some(Line {
            start: a.start,
            end: b.end,
        })
    } else if b.end == a.start {
        Some(Line {
            start: b.start,
            end: a.end,
        })
    } else {
        None
    }
}
fn same_direction(a: Line, b: Line) -> bool {
    let normalized_vector_a = normalized_vector(a);
    let normalized_vector_b = normalized_vector(b);
    let cosine_between_a_b = normalized_vector_a.x * normalized_vector_b.x
        + normalized_vector_a.y * normalized_vector_b.y;
    cosine_between_a_b == 1.0
}
fn normalized_vector(line: Line) -> Coord {
    let vector = line.delta();
    let length = vector.euclidean_distance(&coord! {x: 0., y: 0.});
    vector / length
}

fn get_lines(wall: &Vec<String>, visit_map: &mut Vec<Vec<bool>>, xy: Xy<usize>) -> Vec<Line> {
    match already_visited(&visit_map, xy) {
        true => return Vec::new(),
        false => mark_as_visited(visit_map, xy),
    };
    if !wall_exist(wall, xy) {
        return Vec::new();
    }

    VisitDirection::iter()
        .flat_map(|visit_direction| match visit_direction.next_xy(xy) {
            Some(next_xy) => match wall_exist(wall, next_xy) {
                true => get_lines(wall, visit_map, next_xy),
                false => vec![visit_direction.line(xy)],
            },
            None => vec![visit_direction.line(xy)],
        })
        .collect()
}

enum VisitDirection {
    Left,
    Down,
    Right,
    Up,
}
impl VisitDirection {
    fn iter() -> std::slice::Iter<'static, VisitDirection> {
        const DIRECTIONS: [VisitDirection; 4] = [
            VisitDirection::Left,
            VisitDirection::Down,
            VisitDirection::Right,
            VisitDirection::Up,
        ];
        DIRECTIONS.iter()
    }

    fn next_xy(&self, mut xy: Xy<usize>) -> Option<Xy<usize>> {
        match self {
            VisitDirection::Left => xy.x.checked_sub(1).map(|next_x| {
                xy.x = next_x;
                xy
            }),
            VisitDirection::Down => xy.y.checked_add(1).map(|next_y| {
                xy.y = next_y;
                xy
            }),
            VisitDirection::Right => xy.x.checked_add(1).map(|next_x| {
                xy.x = next_x;
                xy
            }),
            VisitDirection::Up => xy.y.checked_sub(1).map(|next_y| {
                xy.y = next_y;
                xy
            }),
        }
    }

    fn line(&self, xy: Xy<usize>) -> Line {
        let ((start_point_x, start_point_y), (end_point_x, end_point_y)) = match self {
            VisitDirection::Left => ((xy.x, xy.y), (xy.x, xy.y + 1)),
            VisitDirection::Down => ((xy.x, xy.y + 1), (xy.x + 1, xy.y + 1)),
            VisitDirection::Right => ((xy.x + 1, xy.y + 1), (xy.x + 1, xy.y)),
            VisitDirection::Up => ((xy.x + 1, xy.y), (xy.x, xy.y)),
        };
        Line::new(
            coord! {x:start_point_x as f64 - 0.5, y:start_point_y as f64 - 0.5},
            coord! {x:end_point_x as f64 - 0.5, y:end_point_y as f64 - 0.5},
        )
    }
}

fn already_visited(visit_map: &Vec<Vec<bool>>, xy: Xy<usize>) -> bool {
    *visit_map
        .get(xy.y)
        .and_then(|row| row.get(xy.x))
        .unwrap_or(&false)
}

fn mark_as_visited(visit_map: &mut Vec<Vec<bool>>, xy: Xy<usize>) {
    if let Some(visited) = visit_map.get_mut(xy.y).and_then(|row| row.get_mut(xy.x)) {
        *visited = true
    }
}

fn wall_exist(wall: &Vec<String>, xy: Xy<usize>) -> bool {
    wall.get(xy.y)
        .and_then(|row| {
            row.get(xy.x..xy.x + 1).map(|wall| match wall {
                "0" => false,
                _ => true,
            })
        })
        .unwrap_or(false)
}
