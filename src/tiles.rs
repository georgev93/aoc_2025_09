type Coordinate = [i64; 2];

pub struct Floor {
    red_tiles: Vec<Coordinate>,
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

                nums.try_into()
                    .expect("Number of numbers on this line not equal to 2")
            })
            .collect();

        Self { red_tiles }
    }

    pub fn get_largest_rectangle(&self) -> u64 {
        let mut largest: u64 = 0;
        for (coord1_idx, coord1) in self.red_tiles.iter().enumerate() {
            for coord2 in self.red_tiles.iter().skip(coord1_idx + 1) {
                let this_rec_size = Self::get_rectangle_size(coord1, coord2);
                // println!("Evaluated size: {}", this_rec_size);
                if this_rec_size > largest {
                    // println!("New largest: {}", this_rec_size);
                    largest = this_rec_size;
                }
            }
        }
        largest
    }

    fn get_rectangle_size(coord1: &Coordinate, coord2: &Coordinate) -> u64 {
        // println!(
        //     "    Checking size of rec made by ({}, {}) and ({}, {})",
        //     coord1[0], coord1[1], coord2[0], coord2[1]
        // );
        coord1
            .iter()
            .zip(coord2.iter())
            .fold(1u64, |acc, (x, y)| acc * ((x - y).unsigned_abs() + 1))
    }
}
