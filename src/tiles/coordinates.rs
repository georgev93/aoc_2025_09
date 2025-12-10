use core::slice::Iter;
use std::fmt;

#[derive(Copy, Clone, PartialEq)]
pub enum TravelDirection {
    X,
    Y,
    None, // only valid for when no perimeter can pass through the point
}

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
pub struct Coordinate {
    // Treated as [X, Y]
    pub coord: [i64; 2],
}

macro_rules! push_coords_between_on {
    ($coord_start:expr, $coord_stop:expr, $vec:expr) => {
        let (coords, travel_direction) = $coord_start.get_coords_between(&$coord_stop);
        for coord in coords {
            $vec.push((coord, travel_direction));
        }
    };
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.coord[0], self.coord[1])
    }
}

impl fmt::Display for TravelDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TravelDirection::X => write!(f, "X Direction"),
            TravelDirection::Y => write!(f, "Y Direction"),
            TravelDirection::None => write!(f, "None Direction"),
        }
    }
}

impl Coordinate {
    pub fn new(x: i64, y: i64) -> Self {
        Self { coord: [x, y] }
    }

    pub fn from_array(coord: [i64; 2]) -> Self {
        Self { coord }
    }

    pub fn get_rec_area(&self, other: &Self) -> u64 {
        self.coord
            .iter()
            .zip(other.coord.iter())
            .fold(1u64, |acc, (x, y)| acc * ((x - y).unsigned_abs() + 1))
    }

    pub fn get_coords_between(&self, other: &Self) -> (Vec<Self>, TravelDirection) {
        let mut travel_direction = TravelDirection::X;
        if self.coord[0] == other.coord[0] {
            travel_direction = TravelDirection::Y;
        }

        let mut coords_between: Vec<Self>;

        match travel_direction {
            TravelDirection::X => {
                let (min, max) = Self::get_min_max(self.coord[0], other.coord[0]);
                coords_between = Vec::with_capacity((max - min) as usize);
                for val in (min + 1)..max {
                    coords_between.push(Self::new(val, other.coord[1]));
                }
            }
            TravelDirection::Y => {
                let (min, max) = Self::get_min_max(self.coord[1], other.coord[1]);
                coords_between = Vec::with_capacity((max - min) as usize);
                for val in (min + 1)..max {
                    coords_between.push(Self::new(other.coord[0], val));
                }
            }
            _ => {
                unreachable!();
            }
        }

        // println!("Coords between called on {} and {}", &self, other);
        // dbg!(&coords_between);
        (coords_between, travel_direction)
    }

    pub fn get_rec_perimeter(&self, other: &Self) -> Vec<(Self, TravelDirection)> {
        let (x_min, x_max) = Self::get_min_max(self.coord[0], other.coord[0]) as (i64, i64);
        let (y_min, y_max) = Self::get_min_max(self.coord[1], other.coord[1]) as (i64, i64);

        // Single line rectangle
        if (x_min == x_max) || (y_min == y_max) {
            let (ret_vec, travel_direction) = self.get_coords_between(other);
            return ret_vec
                .iter()
                .map(|coord| (*coord, travel_direction))
                .collect();
        }

        let mut ret_vec: Vec<(Self, TravelDirection)> =
            Vec::with_capacity((2 * (x_max - x_min) + 2 * (y_max - y_min) - 4) as usize);

        let top_left_corner = Self::new(x_min, y_min);
        let top_right_corner = Self::new(x_max, y_min);
        let bottom_left_corner = Self::new(x_min, y_max);
        let bottom_right_corner = Self::new(x_max, y_max);

        let (coords, travel_direction) = top_left_corner.get_coords_between(&top_right_corner);
        for coord in coords {
            ret_vec.push((coord, travel_direction));
        }

        push_coords_between_on!(top_left_corner, top_right_corner, ret_vec);
        push_coords_between_on!(top_right_corner, bottom_right_corner, ret_vec);
        push_coords_between_on!(bottom_right_corner, bottom_left_corner, ret_vec);
        push_coords_between_on!(bottom_left_corner, top_left_corner, ret_vec);

        ret_vec
    }

    fn get_min_max<T: Ord>(arg1: T, arg2: T) -> (T, T) {
        let min: T;
        let max: T;
        if arg1 > arg2 {
            (arg2, arg1)
        } else {
            (arg1, arg2)
        }
    }
}
