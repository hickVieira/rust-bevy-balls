pub struct AxisProjection {
    pub sorted: Vec<(usize, f32)>,
    map: Vec<usize>,
    pub ranges: Vec<(usize, usize)>,
    pub length: usize,
}

impl AxisProjection {
    pub fn new(length: usize) -> Self {
        Self {
            sorted: vec![(0, 0.0); length],
            map: vec![0; length],
            ranges: vec![(0, 0); length],
            length,
        }
    }

    pub fn set(&mut self, index: usize, value: f32) {
        self.sorted[index] = (index, value);
    }

    pub fn src_to_proj(&mut self, index: usize) -> usize {
        return self.map[index];
    }

    pub fn proj_to_src(&mut self, index: usize) -> usize {
        return self.sorted[index].0;
    }

    pub fn get_range(&mut self, index: usize) -> (usize, usize) {
        return self.ranges[index];
    }

    pub fn build(&mut self, radius: f32) {
        // sort
        self.sorted.sort_by(|a, b| a.1.total_cmp(&b.1));

        // buid map
        for i in 0..self.length {
            self.map[self.sorted[i].0] = i;
        }

        // figure out the intersection ranges
        for i in 0..self.length {
            self.ranges[i] = self.find_ranges(i, radius);
        }
    }

    fn find_ranges(&self, index: usize, radius: f32) -> (usize, usize) {
        let start = self.sorted[index];
        let mut range: (isize, isize) = (0, 0);

        // search left
        for i in (0..index).rev() {
            if (self.sorted[i].1 - start.1).abs() <= radius {
                range.0 -= 1;
            } else {
                break;
            }
        }

        // search right
        for i in index + 1..self.length {
            if (self.sorted[i].1 - start.1).abs() <= radius {
                range.1 += 1;
            } else {
                break;
            }
        }

        let start_index = index as isize - range.0;
        let end_index = index as isize + range.1 + 1;

        return (start_index as usize, end_index as usize);
    }
}
