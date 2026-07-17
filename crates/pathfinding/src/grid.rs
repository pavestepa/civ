use civ_hex::HexCoordinate;
use pathfinding::prelude::astar;

pub struct HexPathfinder;

impl HexPathfinder {
    pub fn find_path(start: HexCoordinate, goal: HexCoordinate) -> Option<Vec<HexCoordinate>> {
        let result = astar(
            &start,
            |current| {
                current
                    .neighbors()
                    .into_iter()
                    .map(|n| (n, 1u32))
                    .collect::<Vec<_>>()
            },
            |current| current.distance(goal),
            |current| current == &goal,
        );

        result.map(|(path, _)| path)
    }
}
