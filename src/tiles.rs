mod coordinates;
use crate::tiles::coordinates::*;

use std::collections::HashMap;

pub struct Floor {
    red_tiles: Vec<Coordinate>,
    perimeter: HashMap<Coordinate, TravelDirection>,
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

        Self {
            perimeter: HashMap::new(),
            red_tiles,
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
        self.set_usable_tile_perimeter();
        let mut rectangle_counter = 0u64;
        let num_of_rec =
            self.red_tiles.len() * (self.red_tiles.len() / 2) - (self.red_tiles.len() / 2);

        let mut rectangles: Vec<(i64, (&Coordinate, &Coordinate))> = Vec::with_capacity(num_of_rec);

        for (coord1_idx, coord1) in self.red_tiles.iter().enumerate() {
            for coord2 in self.red_tiles.iter().skip(coord1_idx + 1) {
                rectangle_counter += 1;
                println!(
                    "Calculating rectangle {} out of {}",
                    rectangle_counter, num_of_rec
                );
                rectangles.push((coord1.get_rec_area(coord2) as i64, (coord1, coord2)));
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

    fn set_usable_tile_perimeter(&mut self) {
        let mut input_coord_iter = self.red_tiles.iter();
        let mut last_coord = input_coord_iter.next().unwrap();
        for coord in input_coord_iter {
            let (coords_between, travel_direction) = coord.get_coords_between(last_coord);
            for coord_between in coords_between {
                // println!(
                //     "Perimeter entry at {}. Travel direction: {}",
                //     coord_between, travel_direction
                // );
                self.perimeter.insert(coord_between, travel_direction);
            }

            // Ensure no perimeters can pass through the endpoints
            self.perimeter.insert(*coord, TravelDirection::None);

            last_coord = coord;
        }
    }

    fn is_rectangle_usable(&self, coord1: &Coordinate, coord2: &Coordinate) -> bool {
        for (coord, travel_direction) in coord1.get_rec_perimeter(coord2) {
            if !self.is_coordinate_usable(&coord, travel_direction) {
                return false;
            }
        }
        true
    }

    fn is_coordinate_usable(&self, coord: &Coordinate, travel_direction: TravelDirection) -> bool {
        if let Some(perimeter_direction) = self.perimeter.get(coord)
            && *perimeter_direction != travel_direction
        {
            // println!(
            //     "Coordinate {} is not usable because it is traveling in the {} and this perimeter is in the {}",
            //     coord, travel_direction, perimeter_direction
            // );
            return false;
        }
        true
    }
}
