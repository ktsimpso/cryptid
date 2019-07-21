use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct AxialCoordinate {
    pub q: isize,
    pub r: isize,
}

impl AxialCoordinate {
    fn to_cubic(&self) -> CubicCoordinate {
        CubicCoordinate {
            x: self.q,
            y: -self.q - self.r,
            z: self.r,
        }
    }

    pub fn down(&self) -> AxialCoordinate {
        AxialCoordinate {
            q: self.q + 1,
            r: self.r,
        }
    }

    pub fn right_lower(&self) -> AxialCoordinate {
        AxialCoordinate {
            q: self.q + 1,
            r: self.r - 1,
        }
    }

    pub fn right_upper(&self) -> AxialCoordinate {
        AxialCoordinate {
            q: self.q,
            r: self.r - 1,
        }
    }

    pub fn up(&self) -> AxialCoordinate {
        AxialCoordinate {
            q: self.q - 1,
            r: self.r,
        }
    }

    pub fn left_upper(&self) -> AxialCoordinate {
        AxialCoordinate {
            q: self.q - 1,
            r: self.r + 1,
        }
    }

    pub fn left_lower(&self) -> AxialCoordinate {
        AxialCoordinate {
            q: self.q,
            r: self.r + 1,
        }
    }

    pub fn distance_from(&self, other: AxialCoordinate) -> isize {
        self.to_cubic().distance_from(other.to_cubic())
    }

    pub fn all_hexes_within_distance(&self, distance: isize) -> Vec<AxialCoordinate> {
        self.to_cubic()
            .all_hexes_within_distance(distance)
            .iter()
            .map(CubicCoordinate::to_axial)
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct CubicCoordinate {
    x: isize,
    y: isize,
    z: isize,
}

impl CubicCoordinate {
    fn to_axial(&self) -> AxialCoordinate {
        AxialCoordinate {
            q: self.x,
            r: self.z,
        }
    }

    fn distance_from(&self, other: CubicCoordinate) -> isize {
        [self.x - other.x, self.y - other.y, self.z - other.z]
            .iter()
            .map(|x| x.abs())
            .max()
            .unwrap()
    }

    fn all_hexes_within_distance(&self, distance: isize) -> Vec<CubicCoordinate> {
        (-distance..=distance)
            .into_iter()
            .flat_map(|x| {
                let lower_y = std::cmp::max(-distance, -x - distance);
                let upper_y = std::cmp::min(distance, -x + distance);

                (lower_y..=upper_y).into_iter().map(move |y| {
                    let z = -x - y;
                    CubicCoordinate {
                        x: self.x + x,
                        y: self.y + y,
                        z: self.z + z,
                    }
                })
            })
            .collect()
    }
}

#[derive(Debug)]
pub struct HexGrid<T> {
    grid: HashMap<AxialCoordinate, T>,
}

impl<T> HexGrid<T> {
    pub fn new() -> HexGrid<T> {
        HexGrid {
            grid: HashMap::new(),
        }
    }
    pub fn get(&self, coordinate: &AxialCoordinate) -> Option<&T> {
        self.grid.get(coordinate)
    }

    pub fn insert(&mut self, coordinate: AxialCoordinate, t: T) -> () {
        self.grid.insert(coordinate, t);
    }
}

#[cfg(test)]
mod axial_coordinate_tests {
    use super::*;
    use itertools::Itertools;
    use std::collections::HashSet;
    use std::iter::FromIterator;

    #[test]
    fn to_cubic_then_back_to_axial_is_the_identity() {
        let origin = AxialCoordinate { q: 0, r: 0 };
        assert_eq!(origin.to_cubic().to_axial(), origin);
    }

    #[test]
    fn down_should_be_one_space_away_from_current_coordinate() {
        let origin = AxialCoordinate { q: 0, r: 0 };
        let down = origin.down();
        assert_eq!(down.q, 1);
        assert_eq!(down.r, 0);
        assert_eq!(origin.distance_from(down), 1);
    }

    #[test]
    fn righ_lower_should_be_one_space_away_from_current_coordinate() {
        let origin = AxialCoordinate { q: 0, r: 0 };
        let right_lower = origin.right_lower();
        assert_eq!(right_lower.q, 1);
        assert_eq!(right_lower.r, -1);
        assert_eq!(origin.distance_from(right_lower), 1);
    }

    #[test]
    fn righ_upper_should_be_one_space_away_from_current_coordinate() {
        let origin = AxialCoordinate { q: 0, r: 0 };
        let right_upper = origin.right_upper();
        assert_eq!(right_upper.q, 0);
        assert_eq!(right_upper.r, -1);
        assert_eq!(origin.distance_from(right_upper), 1);
    }

    #[test]
    fn up_should_be_one_space_away_from_current_coordinate() {
        let origin = AxialCoordinate { q: 0, r: 0 };
        let up = origin.up();
        assert_eq!(up.q, -1);
        assert_eq!(up.r, 0);
        assert_eq!(origin.distance_from(up), 1);
    }

    #[test]
    fn left_upper_should_be_one_space_away_from_current_coordinate() {
        let origin = AxialCoordinate { q: 0, r: 0 };
        let left_upper = origin.left_upper();
        assert_eq!(left_upper.q, -1);
        assert_eq!(left_upper.r, 1);
        assert_eq!(origin.distance_from(left_upper), 1);
    }

    #[test]
    fn left_lower_should_be_one_space_away_from_current_coordinate() {
        let origin = AxialCoordinate { q: 0, r: 0 };
        let left_lower = origin.left_lower();
        assert_eq!(left_lower.q, 0);
        assert_eq!(left_lower.r, 1);
        assert_eq!(origin.distance_from(left_lower), 1);
    }

    #[test]
    fn all_adjacent_methods_must_not_be_equal_to_each_other() {
        let origin = AxialCoordinate { q: 0, r: 0 };
        let adjacents = [
            origin.down(),
            origin.right_lower(),
            origin.right_upper(),
            origin.up(),
            origin.left_upper(),
            origin.left_lower(),
        ];

        adjacents
            .into_iter()
            .tuple_combinations::<(_, _)>()
            .for_each(|(a, b)| assert_ne!(a, b));
    }

    #[test]
    fn distance_from_must_increase_by_one_every_time_a_direction_is_moved() {
        let origin = AxialCoordinate { q: 0, r: 0 };

        struct AxialCoordinateIterator {
            coordinate: AxialCoordinate,
        }

        impl Iterator for AxialCoordinateIterator {
            type Item = AxialCoordinate;

            fn next(&mut self) -> Option<Self::Item> {
                self.coordinate = self.coordinate.down();
                Some(self.coordinate)
            }
        }

        let coordinates = AxialCoordinateIterator { coordinate: origin };

        coordinates
            .zip(1..)
            .take(10)
            .for_each(|(coordinate, distance)| {
                assert_eq!(origin.distance_from(coordinate), distance)
            });
    }

    #[test]
    fn all_hexes_within_distance_should_return_the_original_hex_if_zero() {
        let origin = AxialCoordinate { q: 0, r: 0 };
        let result = origin.all_hexes_within_distance(0);

        assert_eq!(result.len(), 1);
        assert_eq!(result.get(0).unwrap(), &origin);
    }

    #[test]
    fn all_hexes_within_distance_should_return_the_original_hex_and_surrounding_hexes_if_one() {
        let origin = AxialCoordinate { q: 0, r: 0 };
        let expected: HashSet<AxialCoordinate> = HashSet::from_iter(
            [
                origin,
                origin.down(),
                origin.right_lower(),
                origin.right_upper(),
                origin.up(),
                origin.left_upper(),
                origin.left_lower(),
            ]
            .to_vec(),
        );
        let result: HashSet<AxialCoordinate> =
            HashSet::from_iter(origin.all_hexes_within_distance(1).into_iter());

        assert_eq!(result.len(), 7);
        assert_eq!(result, expected);
    }

    #[test]
    fn all_hexes_within_distance_should_return_19_hexes_if_two() {
        let origin = AxialCoordinate { q: 0, r: 0 };;
        let result: HashSet<AxialCoordinate> =
            HashSet::from_iter(origin.all_hexes_within_distance(2).into_iter());
        let distances = result
            .into_iter()
            .map(|coordinate| origin.distance_from(coordinate))
            .collect::<Vec<_>>();

        assert_eq!(distances.len(), 19);
        assert_eq!(
            distances
                .clone()
                .into_iter()
                .filter(|x| *x == 0)
                .collect::<Vec<isize>>()
                .len(),
            1
        );
        assert_eq!(
            distances
                .clone()
                .into_iter()
                .filter(|x| *x == 1)
                .collect::<Vec<isize>>()
                .len(),
            6
        );
        assert_eq!(
            distances
                .into_iter()
                .filter(|x| *x == 2)
                .collect::<Vec<isize>>()
                .len(),
            12
        );
    }
}
