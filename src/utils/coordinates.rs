use core::slice::Iter;
use itertools::Itertools;
use std::fmt;
use std::fmt::Display;

use std::convert::TryInto;
use std::fmt::Debug;

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
            .fold(1u64, |acc, (a, b)| acc * ((a - b).unsigned_abs() + 1))
    }

    #[inline(always)]
    pub fn x(&self) -> i64 {
        self.coord[0]
    }

    #[inline(always)]
    pub fn y(&self) -> i64 {
        self.coord[1]
    }

    pub fn get_coords_between(&self, other: &Self) -> Vec<Self> {
        let mut travel_direction = TravelDirection::X;
        if self.x() == other.x() {
            travel_direction = TravelDirection::Y;
        }

        let mut coords_between: Vec<Self>;

        match travel_direction {
            TravelDirection::X => {
                let (min, max) = Self::get_min_max(self.x(), other.x());
                coords_between = Vec::with_capacity((max - min) as usize);
                for val in (min + 1)..max {
                    coords_between.push(Self::new(val, self.y()));
                }
            }
            TravelDirection::Y => {
                let (min, max) = Self::get_min_max(self.y(), other.y());
                coords_between = Vec::with_capacity((max - min) as usize);
                for val in (min + 1)..max {
                    coords_between.push(Self::new(other.x(), val));
                }
            }
            _ => {
                unreachable!();
            }
        }

        coords_between
    }

    pub fn get_rec_inner_perimeter(&self, other: &Self) -> Option<Vec<Self>> {
        let (mut x_min, mut x_max) = Self::get_min_max(self.x(), other.x());
        let (mut y_min, mut y_max) = Self::get_min_max(self.y(), other.y());

        if (x_min == x_max) || (y_min == y_max) {
            return None;
        }

        x_max -= 1;
        y_max -= 1;
        x_min += 1;
        y_min += 1;

        // Single line rectangle
        if x_min == x_max {
            let mut ret_vec =
                Coordinate::new(x_min, y_min).get_coords_between(&Coordinate::new(x_min, y_max));
            ret_vec.push(Coordinate::new(x_min, y_min));
            ret_vec.push(Coordinate::new(x_min, y_max));
            return Some(ret_vec);
        }

        if y_min == y_max {
            let mut ret_vec =
                Coordinate::new(x_min, y_min).get_coords_between(&Coordinate::new(x_max, y_min));
            ret_vec.push(Coordinate::new(x_min, y_min));
            ret_vec.push(Coordinate::new(x_max, y_min));
            return Some(ret_vec);
        }

        let mut ret_vec: Vec<Self> =
            Vec::with_capacity(((x_max - x_min) * 2 + (y_max - y_min) * 2) as usize);

        let corners_arr = [
            Self::new(x_min, y_min),
            Self::new(x_min, y_max),
            Self::new(x_max, y_max),
            Self::new(x_max, y_min),
        ];

        corners_arr
            .iter()
            .circular_tuple_windows()
            .for_each(|(a, b)| {
                ret_vec.extend(a.get_coords_between(b).into_iter().collect::<Vec<Self>>())
            });

        ret_vec.push(Coordinate::new(x_min, y_max));
        ret_vec.push(Coordinate::new(x_min, y_min));
        ret_vec.push(Coordinate::new(x_max, y_min));
        ret_vec.push(Coordinate::new(x_max, y_max));

        Some(ret_vec)
    }

    #[inline(always)]
    fn get_min_max<T>(arg1: T, arg2: T) -> (T, T)
    where
        T: PartialOrd,
    {
        let min: T;
        let max: T;
        if arg1 > arg2 {
            (arg2, arg1)
        } else {
            (arg1, arg2)
        }
    }
}
