mod coordinates;
use crate::tiles::coordinates::*;

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

        let mut x_values: Vec<i64> = red_tiles.iter().map(|coord| coord.coord[0]).collect();
        x_values.sort();
        x_values.dedup();

        let mut y_values: Vec<i64> = red_tiles.iter().map(|coord| coord.coord[1]).collect();
        y_values.sort();
        y_values.dedup();

        let mut compressed_grid: Vec<Vec<TileColor>> =
            vec![vec![TileColor::White; x_values.len()]; y_values.len()];

        for red_tile in &red_tiles {
            let mapped_x = x_values.binary_search(&red_tile.coord[0]).unwrap();
            let mapped_y = y_values.binary_search(&red_tile.coord[1]).unwrap();
            compressed_grid[mapped_y][mapped_x] = TileColor::Red;
        }

        Self {
            red_tiles,
            compression_x_mapping: x_values,
            compression_y_mapping: y_values,
            compressed_grid,
        }
    }

    fn make_green_tiles(&mut self) {
        // println!("{:?}", self.compressed_grid);

        let compressed_grid_clone = self.compressed_grid.clone();

        for (y, row) in compressed_grid_clone.iter().enumerate() {
            let mut in_bounds = false;

            for (x, this_tile_color) in row.iter().enumerate() {
                if *this_tile_color == TileColor::Red {
                    in_bounds = !in_bounds;
                } else {
                    self.compressed_grid[y][x] = TileColor::Green;
                }
            }
        }

        for (y, row) in compressed_grid_clone.iter().enumerate() {
            let mut last_tile_color = TileColor::White;
            let mut in_bounds = false;

            for (x, this_tile_color) in row.iter().enumerate() {
                if (*this_tile_color != TileColor::White) {
                    in_bounds = true;
                } else if (*this_tile_color == TileColor::White) && in_bounds {
                    self.compressed_grid[y][x] = TileColor::Green;
                } else if (last_tile_color != TileColor::White)
                    && (*this_tile_color == TileColor::White)
                {
                    in_bounds = false;
                }
                last_tile_color = *this_tile_color;
            }
        }

        // println!("{:?}", self.compressed_grid);
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
        self.make_green_tiles();
        let mut rectangle_counter = 0u64;
        let num_of_rec =
            self.red_tiles.len() * (self.red_tiles.len() / 2) - (self.red_tiles.len() / 2);

        let mut rectangles: Vec<(i64, (&Coordinate, &Coordinate))> = Vec::with_capacity(num_of_rec);

        for (coord1_idx, coord1) in self.red_tiles.iter().enumerate() {
            for coord2 in self.red_tiles.iter().skip(coord1_idx + 1) {
                rectangle_counter += 1;
                let size = coord1.get_rec_area(coord2) as i64;
                rectangles.push((size, (coord1, coord2)));
                // println!(
                //     "Calculating rectangle {} out of {} at coords {}, {} assessed to have size {}",
                //     rectangle_counter, num_of_rec, coord1, coord2, size
                // );
            }
        }
        rectangles.sort_unstable_by_key(|e| -e.0);

        rectangle_counter = 0;
        for rec in rectangles {
            rectangle_counter += 1;
            // println!(
            //     "Evaluating rectangle {} out of {} with size {}",
            //     rectangle_counter, num_of_rec, rec.0
            // );
            if self.is_rectangle_usable(rec.1.0, rec.1.1) {
                return rec.0 as u64;
            }
        }
        0
    }

    fn is_rectangle_usable(&self, coord1: &Coordinate, coord2: &Coordinate) -> bool {
        // println!("Examining rectangle at coords {}, {}", coord1, coord2);

        let [x1, y1] = coord1.coord;
        let [x2, y2] = coord2.coord;

        let xmin: i64;
        let ymin: i64;
        let xmax: i64;
        let ymax: i64;

        if x1 > x2 {
            xmax = x1;
            xmin = x2;
        } else {
            xmax = x2;
            xmin = x1;
        }

        if y1 > y2 {
            ymax = y1;
            ymin = y2;
        } else {
            ymax = y2;
            ymin = y1;
        }

        let xmin_mapped = self.compression_x_mapping.binary_search(&xmin).unwrap();
        let xmax_mapped = self.compression_x_mapping.binary_search(&xmax).unwrap();
        let ymin_mapped = self.compression_y_mapping.binary_search(&ymin).unwrap();
        let ymax_mapped = self.compression_y_mapping.binary_search(&ymax).unwrap();

        for row in &self.compressed_grid[ymin_mapped..=ymax_mapped] {
            for tile in &row[xmin_mapped..=xmax_mapped] {
                // dbg!(tile);
                if *tile == TileColor::White {
                    return false;
                }
            }
        }
        true
    }
}
