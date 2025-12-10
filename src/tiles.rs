use crate::utils::coordinates::*;

use std::fmt;

#[derive(Clone, Copy, PartialEq)]
enum TileColor {
    Red,
    Green,
    White,
}

impl fmt::Display for TileColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TileColor::Red => write!(f, "x"),
            TileColor::Green => write!(f, "o"),
            TileColor::White => write!(f, "."),
        }
    }
}

impl fmt::Debug for TileColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TileColor::Red => write!(f, "x"),
            TileColor::Green => write!(f, "o"),
            TileColor::White => write!(f, "."),
        }
    }
}

pub struct Floor {
    red_tiles: Vec<Coordinate>,
    red_tiles_compressed: Vec<Coordinate>,
    compressed_grid: Vec<Vec<TileColor>>,
    compression_x_mapping: Vec<i64>,
    compression_y_mapping: Vec<i64>,
}

impl Floor {
    pub fn new(input: &str) -> Self {
        let red_tiles: Vec<Coordinate> = input
            .lines()
            .map(|line| {
                let nums = line
                    .split(',')
                    .map(|num| num.trim().parse::<i64>().expect("Failed to parse i32"))
                    .collect::<Vec<i64>>();

                Coordinate::from_array(
                    nums.try_into()
                        .expect("Error: Unable to turn line into [i64; 2]"),
                )
            })
            .collect();

        let red_tiles_compressed: Vec<Coordinate> = Vec::with_capacity(red_tiles.len());

        let mut x_values: Vec<i64> = red_tiles.iter().map(|coord| coord.coord[0]).collect();
        x_values.sort();
        x_values.dedup();

        let mut y_values: Vec<i64> = red_tiles.iter().map(|coord| coord.coord[1]).collect();
        y_values.sort();
        y_values.dedup();

        let compressed_grid: Vec<Vec<TileColor>> =
            vec![vec![TileColor::White; x_values.len()]; y_values.len()];

        let mut ret_val = Self {
            red_tiles,
            red_tiles_compressed,
            compression_x_mapping: x_values,
            compression_y_mapping: y_values,
            compressed_grid,
        };

        ret_val.compress_red_tile_coords();

        ret_val
    }

    fn compress_red_tile_coords(&mut self) {
        self.red_tiles_compressed = self.red_tiles.iter().map(|t| self.compress(t)).collect();
    }

    fn make_perimeter(&mut self) {
        let mut red_tile_iter = self.red_tiles_compressed.iter();

        let mut last_tile = red_tile_iter.next().unwrap();
        self.compressed_grid[last_tile.y() as usize][last_tile.x() as usize] = TileColor::Red;

        for red_tile in red_tile_iter {
            self.compressed_grid[red_tile.y() as usize][red_tile.x() as usize] = TileColor::Red;
            for coord in red_tile.get_coords_between(last_tile) {
                self.compressed_grid[coord.y() as usize][coord.x() as usize] = TileColor::Green;
            }
            last_tile = red_tile;
        }
    }

    pub fn get_largest_rectangle(&self) -> u64 {
        let mut largest: u64 = 0;
        for (coord1_idx, coord1) in self.red_tiles.iter().enumerate() {
            for coord2 in self.red_tiles.iter().skip(coord1_idx + 1) {
                let this_rec_size = coord1.get_rec_area(coord2);
                // println!("Evaluated size: {}", this_rec_size);
                if this_rec_size > largest {
                    // println!("New largest: {}", this_rec_size);
                    largest = this_rec_size;
                }
            }
        }
        largest
    }

    pub fn get_largest_rectangle_inside_perimeter(&mut self) -> u64 {
        self.make_perimeter();
        let mut rectangle_counter = 0u64;
        let num_of_rec = self.red_tiles.len() * (self.red_tiles.len() - 1) / 2;

        let mut rectangles: Vec<(i64, (&Coordinate, &Coordinate))> = Vec::with_capacity(num_of_rec);

        for (coord1_idx, coord1) in self.red_tiles_compressed.iter().enumerate() {
            for coord2 in self.red_tiles_compressed.iter().skip(coord1_idx + 1) {
                rectangle_counter += 1;
                let decompressed_coord1 = self.decompress(coord1);
                let decompressed_coord2 = self.decompress(coord2);
                let size = decompressed_coord1.get_rec_area(&decompressed_coord2) as i64;
                rectangles.push((size, (coord1, coord2)));
                // println!(
                //     "Calculating rectangle {} out of {} at coords {}, {} assessed to have size {}",
                // rectangle_counter, num_of_rec, decompressed_coord1, decompressed_coord2, size
                // );
            }
        }
        rectangles.sort_unstable_by_key(|e| -e.0);

        rectangle_counter = 0;
        for rec in rectangles {
            rectangle_counter += 1;
            println!(
                "Evaluating rectangle {} out of {} with size {}",
                rectangle_counter, num_of_rec, rec.0
            );
            if self.is_rectangle_usable(rec.1.0, rec.1.1) {
                return rec.0 as u64;
            }
        }
        0
    }

    fn is_rectangle_usable(&self, coord1: &Coordinate, coord2: &Coordinate) -> bool {
        // println!("Examining rectangle at coords {}, {}", coord1, coord2);
        // dbg!(&self.compressed_grid);
        let inner_perim_coords = coord1.get_rec_inner_perimeter(coord2);

        for coord in inner_perim_coords {
            if self.compressed_grid[coord.y() as usize][coord.x() as usize] != TileColor::White {
                return false;
            }
        }
        true
    }

    fn compress(&self, uncompressed: &Coordinate) -> Coordinate {
        Coordinate::new(
            self.compression_x_mapping
                .binary_search(&uncompressed.x())
                .unwrap()
                .try_into()
                .unwrap(),
            self.compression_y_mapping
                .binary_search(&uncompressed.y())
                .unwrap()
                .try_into()
                .unwrap(),
        )
    }

    fn decompress(&self, compressed: &Coordinate) -> Coordinate {
        Coordinate::new(
            self.compression_x_mapping[compressed.x() as usize],
            self.compression_y_mapping[compressed.y() as usize],
        )
    }
}
