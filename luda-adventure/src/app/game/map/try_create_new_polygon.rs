use crate::app::game::{Edge, Polygon, Tile, TileExt};
use namui::prelude::*;
use std::collections::HashMap;

pub(super) fn try_create_new_polygon(
    wall: &Vec<String>,
    visit_map: &mut Vec<Vec<bool>>,
    start_xy: Xy<usize>,
) -> Option<Polygon> {
    let edges = get_edges(wall, visit_map, start_xy);
    let edges = simplify_edges(edges);
    if edges.len() < 3 {
        return None;
    }
    return None;
}

fn simplify_edges(edges: Vec<Edge>) -> Vec<Edge> {
    let mut start_point_edge_map = HashMap::new();
    let mut simplified_edges = Vec::new();
    for edge in edges.into_iter() {
        let start_point = edge.start_point();
        let key = key_from_point(start_point);
        start_point_edge_map.insert(key, edge);
    }

    while let Some(first_key) = start_point_edge_map.keys().next().copied() {
        let mut merging_edge = start_point_edge_map.remove(&first_key).unwrap();
        loop {
            let next_edge_key = key_from_point(merging_edge.end_point());
            if let Some(next_edge) = start_point_edge_map.remove(&next_edge_key) {
                if merging_edge.is_same_direction_as(&next_edge) {
                    merging_edge =
                        Edge::new_outside(merging_edge.start_point(), next_edge.end_point());
                } else {
                    simplified_edges.push(merging_edge);
                    merging_edge = next_edge;
                }
            } else {
                simplified_edges.push(merging_edge);
                break;
            }
        }
    }

    simplified_edges
}
fn key_from_point(start_point: Xy<Tile>) -> (u32, u32) {
    (
        start_point.x.as_f32().to_bits(),
        start_point.y.as_f32().to_bits(),
    )
}

fn get_edges(wall: &Vec<String>, visit_map: &mut Vec<Vec<bool>>, xy: Xy<usize>) -> Vec<Edge> {
    match already_visited(&visit_map, xy) {
        true => return Vec::new(),
        false => mark_as_visited(visit_map, xy),
    };

    VisitDirection::iter()
        .flat_map(|visit_direction| {
            let next_xy = visit_direction.next_xy(xy);
            match wall_exist(wall, xy) {
                true => get_edges(wall, visit_map, next_xy),
                false => vec![visit_direction.edge(xy)],
            }
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

    fn next_xy(&self, xy: Xy<usize>) -> Xy<usize> {
        match self {
            VisitDirection::Left => xy - Xy::new(1, 0),
            VisitDirection::Down => xy + Xy::new(1, 1),
            VisitDirection::Right => xy + Xy::new(1, 0),
            VisitDirection::Up => xy - Xy::new(0, 1),
        }
    }

    fn edge(&self, xy: Xy<usize>) -> Edge {
        let ((start_point_x, start_point_y), (end_point_x, end_point_y)) = match self {
            VisitDirection::Left => ((xy.x as f32, xy.y as f32), (xy.x as f32, xy.y as f32)),
            VisitDirection::Down => ((xy.x as f32, xy.y as f32), (xy.x as f32, xy.y as f32)),
            VisitDirection::Right => ((xy.x as f32, xy.y as f32), (xy.x as f32, xy.y as f32)),
            VisitDirection::Up => ((xy.x as f32, xy.y as f32), (xy.x as f32, xy.y as f32)),
        };
        Edge::new_outside(
            Xy::new(start_point_x.tile(), start_point_y.tile()),
            Xy::new(end_point_x.tile(), end_point_y.tile()),
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
