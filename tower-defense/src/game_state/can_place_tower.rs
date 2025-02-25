use std::collections::VecDeque;

use super::*;

pub fn can_place_tower(
    tower_left_top: MapCoord,
    tower_size: Wh<usize>,
    travel_points: &[MapCoord],
    placed_tower_coords: &[MapCoord],
    route_coords: &[MapCoord],
    map_wh: Wh<usize>,
) -> bool {
    // NOTE: I guess this function would be called frequently so it needs to be optimized.
    /*
        Algorithm
        1. Check if the new tower disrupts the route.
        2. If it disrupts, divide the route into multiple routes by the new tower.
        3. Let's say the route is divided START->A, A->B, ..., Z->END.
          - Don't think about the exact number of divided routes because only START->A and Z->END are important.
          - You can find A and Z by checking the route from START to END direction, and vice versa.
        4. Perform BFS from A to Z-END. It's the same as performing BFS from START to END, but more efficient.
    */

    let new_tower_coords = (0..tower_size.width)
        .flat_map(|x| (0..tower_size.height).map(move |y| MapCoord::new(x, y) + tower_left_top))
        .collect::<Vec<_>>();

    for new_tower_coord in &new_tower_coords {
        let is_out_of_map = new_tower_coord.x >= map_wh.width || new_tower_coord.y >= map_wh.height;
        let is_collided_to_other_tower = placed_tower_coords.contains(new_tower_coord);
        let is_collided_to_travel_point = travel_points.contains(new_tower_coord);

        if is_out_of_map || is_collided_to_other_tower || is_collided_to_travel_point {
            return false;
        }
    }

    let splitted_routes_by_travel_point = {
        let mut routes = vec![];
        let mut full_route_coords = route_coords.iter().copied().collect::<VecDeque<_>>();
        for i in 0..travel_points.len() - 1 {
            let start = travel_points[i];
            assert_eq!(*full_route_coords.front().unwrap(), start);

            let end = travel_points[i + 1];
            let mut route = vec![];

            loop {
                let coord = full_route_coords.pop_front().unwrap();
                if coord == end {
                    break;
                }
                route.push(coord);
            }

            route.push(end);
            routes.push(route);
        }
        routes
    };

    let disrupts = find_all_disrupts(&new_tower_coords, placed_tower_coords);

    for splitted_route_coords in splitted_routes_by_travel_point {
        let Some(start_side_disrupt_point_index) =
            find_disrupted_route_point_index(splitted_route_coords.iter(), &disrupts)
        else {
            continue;
        };
        if start_side_disrupt_point_index == 0 {
            continue; // NOTE: I'm not sure this is correct. Remove this comment if you can confirm it.
        }
        let end_side_disrupt_after_point_index = splitted_route_coords.len()
            - find_disrupted_route_point_index(splitted_route_coords.iter().rev(), &disrupts)
                .unwrap();

        let start_side_disrupt_before_point = route_coords[start_side_disrupt_point_index - 1];

        let tower_coords_with_new = placed_tower_coords
            .iter()
            .chain(new_tower_coords.iter())
            .copied()
            .collect::<Vec<_>>();

        if crate::route::bfs(
            crate::game_state::MAP_SIZE,
            start_side_disrupt_before_point,
            &splitted_route_coords[end_side_disrupt_after_point_index..],
            &tower_coords_with_new,
        )
        .is_none()
        {
            return false;
        }
    }

    true
}

/// None if no disrupt.
fn find_disrupted_route_point_index<'a>(
    route_coords: impl Iterator<Item = &'a MapCoord> + 'a,
    disrupts: &[Disrupt],
) -> Option<usize> {
    let mut iter = route_coords.enumerate().peekable();
    while let Some((i, route_coord)) = iter.next() {
        let route_coord_i_plus_1 = iter.peek().map(|(_, route_coord)| route_coord);

        for disrupt in disrupts {
            match disrupt {
                Disrupt::One { coord } => {
                    if route_coord == coord {
                        return Some(i);
                    }
                }
                Disrupt::Path { coord1, coord2 } => {
                    let Some(&route_coord_i_plus_1) = route_coord_i_plus_1 else {
                        continue;
                    };
                    if route_coord == coord1 && route_coord_i_plus_1 == coord2
                        || route_coord == coord2 && route_coord_i_plus_1 == coord1
                    {
                        return Some(i);
                    }
                }
            }
        }
    }

    None
}

#[derive(Debug)]
enum Disrupt {
    One { coord: MapCoord },
    Path { coord1: MapCoord, coord2: MapCoord },
}

fn find_all_disrupts(
    new_tower_coords: &[MapCoord],
    placed_tower_coords: &[MapCoord],
) -> Vec<Disrupt> {
    let mut disrupt = vec![];

    for coord in new_tower_coords.iter().cloned() {
        disrupt.push(Disrupt::One { coord });
    }

    for placed_tower_coord in placed_tower_coords {
        for new_tower_coord in new_tower_coords {
            let is_near_diagonal = {
                placed_tower_coord.x.abs_diff(new_tower_coord.x) == 1
                    && placed_tower_coord.y.abs_diff(new_tower_coord.y) == 1
            };
            if !is_near_diagonal {
                continue;
            }

            let coord1 = MapCoord::new(
                placed_tower_coord.x.min(new_tower_coord.x),
                placed_tower_coord.y.min(new_tower_coord.y),
            );
            let coord2 = MapCoord::new(
                placed_tower_coord.x.max(new_tower_coord.x),
                placed_tower_coord.y.max(new_tower_coord.y),
            );

            disrupt.push(Disrupt::Path { coord1, coord2 });
        }
    }

    disrupt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_place_tower() {
        const MAP_SIZE: Wh<BlockUnit> = Wh::new(10, 10);

        /// ```text
        /// ■ ■ ■ ■ ■ ■ ■ ■ ■ ■ ■ ■
        /// ■ 1 ■ ■ ■ 5 ← ← ← ← 4 ■
        /// ■ ↓ ■ ■ ■ ↓ ■ ■ ■ ■ ↑ ■
        /// ■ ↓ ■ ■ ■ ↓ ■ ■ ■ ■ ↑ ■
        /// ■ ↓ ■ ■ ■ ↓ ■ ■ ■ ■ ↑ ■
        /// ■ 2 → → → ┼ → → → → 3 ■
        /// ■ ■ ■ ■ ■ ↓ ■ ■ ■ ■ ■ ■
        /// ■ ■ ■ ■ ■ ↓ ■ ■ ■ ■ ■ ■
        /// ■ ■ ■ ■ ■ ↓ ■ ■ ■ ■ ■ ■
        /// ■ ■ ■ ■ ■ 6 → → → → 7 ■
        /// ■ ■ ■ ■ ■ ■ ■ ■ ■ ■ ■ ■
        const TRAVEL_POINTS: [MapCoord; 7] = [
            MapCoord::new(1, 1),
            MapCoord::new(1, 5),
            MapCoord::new(8, 5),
            MapCoord::new(8, 1),
            MapCoord::new(4, 1),
            MapCoord::new(4, 8),
            MapCoord::new(8, 8),
        ];

        let tower_size = Wh::new(2, 2);

        {
            let mut placed_tower_coords = vec![];
            let mut route =
                crate::route::calculate_routes(&placed_tower_coords, &TRAVEL_POINTS, MAP_SIZE)
                    .unwrap();
            for tower_left_top in [
                Xy { x: 0, y: 6 },
                Xy { x: 2, y: 1 },
                Xy { x: 4, y: 2 },
                Xy { x: 2, y: 6 },
                Xy { x: 3, y: 4 },
                Xy { x: 8, y: 6 },
                Xy { x: 0, y: 3 },
            ] {
                if can_place_tower(
                    tower_left_top,
                    tower_size,
                    &TRAVEL_POINTS,
                    &placed_tower_coords,
                    route.iter_coords(),
                    MAP_SIZE,
                ) {
                    let last_route = route.clone();
                    let last_placed_tower_coords = placed_tower_coords.clone();
                    assert!(tower_left_top.x < MAP_SIZE.width);
                    assert!(tower_left_top.y < MAP_SIZE.height);
                    placed_tower_coords.extend((0..tower_size.width).flat_map(|x| {
                        (0..tower_size.height).map(move |y| MapCoord::new(x, y) + tower_left_top)
                    }));
                    route = crate::route::calculate_routes(
                        &placed_tower_coords,
                        &TRAVEL_POINTS,
                        MAP_SIZE,
                    )
                    .unwrap_or_else(|| {
                        unreachable!(
                            "Failed to calculate route with placed_tower_coords.\n
-last_route-
{}
-new_route-
{}",
                            debug_print_map(
                                MAP_SIZE,
                                &last_placed_tower_coords,
                                &TRAVEL_POINTS,
                                last_route.iter_coords()
                            ),
                            debug_print_map(
                                MAP_SIZE,
                                &placed_tower_coords,
                                &TRAVEL_POINTS,
                                route.iter_coords()
                            )
                        )
                    });
                }
            }
        }

        let mut placed_tower_coords = vec![];
        let mut route =
            crate::route::calculate_routes(&placed_tower_coords, &TRAVEL_POINTS, MAP_SIZE).unwrap();

        assert!(!can_place_tower(
            MapCoord::new(0, MAP_SIZE.height - 1),
            tower_size,
            &TRAVEL_POINTS,
            &placed_tower_coords,
            route.iter_coords(),
            MAP_SIZE,
        ));
        assert!(!can_place_tower(
            MapCoord::new(3, 7),
            tower_size,
            &TRAVEL_POINTS,
            &placed_tower_coords,
            route.iter_coords(),
            MAP_SIZE,
        ));

        let mut tower_placed = false;
        let mut tower_left_top_history = vec![];

        for _ in 0..100 {
            let tower_left_top = MapCoord::new(
                rand::random::<usize>() % MAP_SIZE.width,
                rand::random::<usize>() % MAP_SIZE.height,
            );
            if can_place_tower(
                tower_left_top,
                tower_size,
                &TRAVEL_POINTS,
                &placed_tower_coords,
                route.iter_coords(),
                MAP_SIZE,
            ) {
                let last_route = route.clone();
                let last_placed_tower_coords = placed_tower_coords.clone();
                assert!(tower_left_top.x < MAP_SIZE.width);
                assert!(tower_left_top.y < MAP_SIZE.height);
                tower_left_top_history.push(tower_left_top);
                tower_placed = true;
                placed_tower_coords.extend((0..tower_size.width).flat_map(|x| {
                    (0..tower_size.height).map(move |y| MapCoord::new(x, y) + tower_left_top)
                }));
                route =
                    crate::route::calculate_routes(&placed_tower_coords, &TRAVEL_POINTS, MAP_SIZE)
                        .unwrap_or_else(|| {
                            unreachable!(
                                "Failed to calculate route with placed_tower_coords.\n
-last_route-
{}
-new_route-
{}
tower_left_top_history: {tower_left_top_history:?}",
                                debug_print_map(
                                    MAP_SIZE,
                                    &last_placed_tower_coords,
                                    &TRAVEL_POINTS,
                                    last_route.iter_coords()
                                ),
                                debug_print_map(
                                    MAP_SIZE,
                                    &placed_tower_coords,
                                    &TRAVEL_POINTS,
                                    route.iter_coords()
                                )
                            )
                        });
            }
        }

        assert!(tower_placed);
    }

    #[test]
    /// https://github.com/NamseEnt/namseent/issues/1034
    fn issue_1034() {
        const MAP_SIZE: Wh<BlockUnit> = Wh::new(48, 48);
        const TRAVEL_POINTS: [MapCoord; 7] = [
            MapCoord::new(6, 0),
            MapCoord::new(6, 23),
            MapCoord::new(41, 23),
            MapCoord::new(41, 6),
            MapCoord::new(24, 6),
            MapCoord::new(24, 41),
            MapCoord::new(47, 41),
        ];

        let tower_size = Wh::new(2, 2);

        let placed_tower_coords = vec![];
        let route =
            crate::route::calculate_routes(&placed_tower_coords, &TRAVEL_POINTS, MAP_SIZE).unwrap();

        assert!(can_place_tower(
            MapCoord::new(4, 2),
            tower_size,
            &TRAVEL_POINTS,
            &placed_tower_coords,
            route.iter_coords(),
            MAP_SIZE,
        ));

        assert!(can_place_tower(
            MapCoord::new(5, 2),
            tower_size,
            &TRAVEL_POINTS,
            &placed_tower_coords,
            route.iter_coords(),
            MAP_SIZE,
        ));
    }

    #[test]
    /// https://private-user-images.githubusercontent.com/38313680/416219582-8ddd40ea-d919-4cff-90f1-d3758290a315.png?jwt=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3NDA0NTk1MjEsIm5iZiI6MTc0MDQ1OTIyMSwicGF0aCI6Ii8zODMxMzY4MC80MTYyMTk1ODItOGRkZDQwZWEtZDkxOS00Y2ZmLTkwZjEtZDM3NTgyOTBhMzE1LnBuZz9YLUFtei1BbGdvcml0aG09QVdTNC1ITUFDLVNIQTI1NiZYLUFtei1DcmVkZW50aWFsPUFLSUFWQ09EWUxTQTUzUFFLNFpBJTJGMjAyNTAyMjUlMkZ1cy1lYXN0LTElMkZzMyUyRmF3czRfcmVxdWVzdCZYLUFtei1EYXRlPTIwMjUwMjI1VDA0NTM0MVomWC1BbXotRXhwaXJlcz0zMDAmWC1BbXotU2lnbmF0dXJlPWYzNTc5OTNhYjVlYmUzZDUxYTAwZTA2MzY2OThjMjJlYTM3YTE0ZjhlNjE0ZTgxOTMzYTk3OGU5M2JiZTUwYzgmWC1BbXotU2lnbmVkSGVhZGVycz1ob3N0In0.UvTwIOPwvH-NOmbsYXWXpXE423lIH_dMLDhZ-1KAIxE
    fn blocking_start_point_should_not_be_allowed() {
        const MAP_SIZE: Wh<BlockUnit> = Wh::new(48, 48);
        const TRAVEL_POINTS: [MapCoord; 7] = [
            MapCoord::new(6, 0),
            MapCoord::new(6, 23),
            MapCoord::new(41, 23),
            MapCoord::new(41, 6),
            MapCoord::new(24, 6),
            MapCoord::new(24, 41),
            MapCoord::new(47, 41),
        ];

        let tower_size = Wh::new(2, 2);

        let placed_tower_coords = vec![
            MapCoord::new(4, 0),
            MapCoord::new(4, 1),
            MapCoord::new(5, 0),
            MapCoord::new(5, 1),
            MapCoord::new(8, 0),
            MapCoord::new(8, 1),
            MapCoord::new(9, 0),
            MapCoord::new(9, 1),
        ];
        let route =
            crate::route::calculate_routes(&placed_tower_coords, &TRAVEL_POINTS, MAP_SIZE).unwrap();

        assert!(!can_place_tower(
            MapCoord::new(6, 1),
            tower_size,
            &TRAVEL_POINTS,
            &placed_tower_coords,
            route.iter_coords(),
            MAP_SIZE,
        ));
    }

    #[test]
    fn test_find_disrupted_route_point_index() {
        let disrupts = vec![
            Disrupt::One {
                coord: MapCoord::new(1, 1),
            },
            Disrupt::Path {
                coord1: MapCoord::new(2, 2),
                coord2: MapCoord::new(3, 3),
            },
        ];

        let route_coords = vec![
            MapCoord::new(0, 0),
            MapCoord::new(1, 1),
            MapCoord::new(2, 2),
            MapCoord::new(3, 3),
            MapCoord::new(4, 4),
        ];

        assert_eq!(
            find_disrupted_route_point_index(route_coords.iter(), &disrupts),
            Some(1)
        );
    }

    fn debug_print_map(
        map_wh: Wh<usize>,
        placed_tower_coords: &[MapCoord],
        travel_points: &[MapCoord],
        route_coords: &[MapCoord],
    ) -> String {
        let mut output = String::new();
        for y in 0..map_wh.height {
            for x in 0..map_wh.width {
                let coord = MapCoord::new(x, y);
                if placed_tower_coords.contains(&coord) {
                    output.push('T');
                } else if travel_points.contains(&coord) {
                    output.push('⊙');
                } else if route_coords.contains(&coord) {
                    output.push('→');
                } else {
                    output.push(' ');
                }
            }
            output.push('\n');
        }
        output
    }
}
