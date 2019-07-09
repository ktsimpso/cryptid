use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
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
