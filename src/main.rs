mod hex;
use hex::AxialCoordinate;
use hex::HexGrid;

#[derive(Debug)]
enum Terrain {
    Water,
    Mountain,
    Forest,
    Swamp,
    Desert,
}

#[derive(Debug)]
enum Animal {
    Bear,
    Cougar,
}

#[derive(Debug)]
enum StructureColor {
    Blue,
    Green,
    White,
    Black,
}

#[derive(Debug)]
enum Structure {
    StandingStone(StructureColor),
    AdbandonedShack(StructureColor),
}

#[derive(Debug)]
struct Tile {
    terrian: Terrain,
    animal: Option<Animal>,
    structure: Option<Structure>,
}

fn main() {
    use crate::Animal::*;
    use crate::Structure::*;
    use crate::StructureColor::*;
    use crate::Terrain::*;

    let t = Tile {
        terrian: Water,
        animal: Some(Bear),
        structure: Some(StandingStone(Green)),
    };

    let coord = AxialCoordinate { q: 0, r: 0 };
    let coord2 = coord.left_upper().down().right_upper();

    let coords = coord.all_hexes_within_distance(1);

    let mut grid: HexGrid<Tile> = HexGrid::new();

    println!("{:?}", t);
    println!("{:?}", coord);
    println!("{:?}", coord2);
    println!("{:?}", coord.distance_from(coord2));
    println!("{:?}", coords);

    grid.insert(coord, t);

    println!("{:?}", grid);
}
