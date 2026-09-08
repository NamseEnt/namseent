use super::*;

pub fn can_place_tower(
    tower_left_top: MapCoord,
    tower_size: Wh<usize>,
    travel_points: &[MapCoord],
    placed_tower_coords: &[MapCoord],
    route_coords: &[MapCoord],
    map_wh: Wh<usize>,
) -> bool {
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

    let disrupts = find_all_disrupts(&new_tower_coords, placed_tower_coords);

    let all_tower_coords = {
        let mut coords = new_tower_coords;
        coords.extend_from_slice(placed_tower_coords);
        coords
    };

    let mut route_coords_queue = route_coords;
    for i in 0..travel_points.len() - 1 {
        let start = travel_points[i];
        let end = travel_points[i + 1];

        let section_route_coords = {
            let index = route_coords_queue
                .iter()
                .position(|&coord| coord == end)
                .unwrap();
            let left = &route_coords_queue[index..];
            let right = &route_coords_queue[..index + 1];
            route_coords_queue = left;
            right
        };

        if !is_disrupted(section_route_coords, &disrupts) {
            continue;
        }

        let new_route = crate::route::find_shortest_route(map_wh, start, end, &all_tower_coords);

        if new_route.is_none() {
            return false;
        }
    }

    true
}

#[derive(Debug, PartialEq, State)]
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

            let coord1 = MapCoord::new(placed_tower_coord.x, new_tower_coord.y);
            let coord2 = MapCoord::new(new_tower_coord.x, placed_tower_coord.y);

            disrupt.push(Disrupt::Path { coord1, coord2 });
        }
    }

    disrupt
}

fn is_disrupted(route_coords: &[MapCoord], disrupts: &[Disrupt]) -> bool {
    disrupts.iter().any(|disrupt| match disrupt {
        Disrupt::One { coord } => route_coords.contains(coord),
        &Disrupt::Path { coord1, coord2 } => route_coords.windows(2).any(|window| {
            window[0] == coord1 && window[1] == coord2 || window[0] == coord2 && window[1] == coord1
        }),
    })
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

        for _ in 0..100 {
            let mut placed_tower_coords = vec![];
            let mut route =
                crate::route::calculate_routes(&placed_tower_coords, &TRAVEL_POINTS, MAP_SIZE)
                    .unwrap();
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
    }

    #[test]
    fn test_empty_map() {
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
        let placed_tower_coords = vec![];
        let route =
            crate::route::calculate_routes(&placed_tower_coords, &TRAVEL_POINTS, MAP_SIZE).unwrap();

        assert_eq!(route.iter_coords()[0], MapCoord::new(1, 1));

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
    }

    #[test]
    fn test_can_place_tower_case1() {
        const MAP_SIZE: Wh<BlockUnit> = Wh::new(10, 10);
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

        let mut placed_tower_coords = vec![];
        let mut route =
            crate::route::calculate_routes(&placed_tower_coords, &TRAVEL_POINTS, MAP_SIZE).unwrap();
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
                route =
                    crate::route::calculate_routes(&placed_tower_coords, &TRAVEL_POINTS, MAP_SIZE)
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
    /// https://github.com/NamseEnt/namseent/issues/1048
    fn issue_1048() {
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
            MapCoord::new(7, 0),
            MapCoord::new(7, 1),
            MapCoord::new(8, 0),
            MapCoord::new(8, 1),
            MapCoord::new(5, 2),
            MapCoord::new(5, 3),
            MapCoord::new(6, 2),
            MapCoord::new(6, 3),
            MapCoord::new(3, 4),
            MapCoord::new(3, 5),
            MapCoord::new(4, 4),
            MapCoord::new(4, 5),
        ];
        let route =
            crate::route::calculate_routes(&placed_tower_coords, &TRAVEL_POINTS, MAP_SIZE).unwrap();

        assert!(!can_place_tower(
            MapCoord::new(3, 0),
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

    /// https://github.com/NamseEnt/namseent/actions/runs/16744617267/job/47400614059#step:7:1036
    fn cicd_47400614059() {
        const MAP_SIZE: Wh<BlockUnit> = Wh::new(48, 48);
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

        let placed_tower_coords = [
            MapCoord::new(0, 2),
            MapCoord::new(6, 8),
            MapCoord::new(5, 3),
            MapCoord::new(8, 2),
            MapCoord::new(1, 8),
            MapCoord::new(6, 6),
            MapCoord::new(3, 6),
        ]
        .into_iter()
        .flat_map(|coord| {
            (coord.x..coord.x + tower_size.width).flat_map(move |x| {
                (coord.y..coord.y + tower_size.height).map(move |y| MapCoord::new(x, y))
            })
        })
        .collect::<Vec<_>>();
        let route =
            crate::route::calculate_routes(&placed_tower_coords, &TRAVEL_POINTS, MAP_SIZE).unwrap();

        assert!(!can_place_tower(
            MapCoord::new(2, 0),
            tower_size,
            &TRAVEL_POINTS,
            &placed_tower_coords,
            route.iter_coords(),
            MAP_SIZE,
        ));
    }

    #[test]
    fn find_all_disrupts_47400614059() {
        assert_eq!(
            find_all_disrupts(
                &[
                    MapCoord::new(2, 0),
                    MapCoord::new(2, 1),
                    MapCoord::new(3, 0),
                    MapCoord::new(3, 1),
                ],
                &[
                    MapCoord::new(0, 2),
                    MapCoord::new(0, 3),
                    MapCoord::new(1, 2),
                    MapCoord::new(1, 3),
                ],
            ),
            vec![
                Disrupt::One {
                    coord: MapCoord::new(2, 0),
                },
                Disrupt::One {
                    coord: MapCoord::new(2, 1),
                },
                Disrupt::One {
                    coord: MapCoord::new(3, 0),
                },
                Disrupt::One {
                    coord: MapCoord::new(3, 1),
                },
                Disrupt::Path {
                    coord1: MapCoord::new(1, 1),
                    coord2: MapCoord::new(2, 2),
                },
            ]
        );
    }

    #[test]
    fn find_all_disrupts_diagonal_left_bottom() {
        assert_eq!(
            find_all_disrupts(
                &[
                    MapCoord::new(2, 0),
                    MapCoord::new(2, 1),
                    MapCoord::new(3, 0),
                    MapCoord::new(3, 1),
                ],
                &[
                    MapCoord::new(0, 2),
                    MapCoord::new(0, 3),
                    MapCoord::new(1, 2),
                    MapCoord::new(1, 3),
                ],
            ),
            vec![
                Disrupt::One {
                    coord: MapCoord::new(2, 0),
                },
                Disrupt::One {
                    coord: MapCoord::new(2, 1),
                },
                Disrupt::One {
                    coord: MapCoord::new(3, 0),
                },
                Disrupt::One {
                    coord: MapCoord::new(3, 1),
                },
                Disrupt::Path {
                    coord1: MapCoord::new(1, 1),
                    coord2: MapCoord::new(2, 2),
                },
            ]
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
