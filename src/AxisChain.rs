pub struct AxisChain {
    radius: f32,
    pairs: Vec<(usize, f32)>,
    map: Vec<usize>,
    length: usize,
}

impl AxisChain {
    pub fn new(radius: f32, length: usize) -> Self {
        Self {
            radius,
            pairs: vec![(0, 0.0); length],
            map: vec![0; length],
            length,
        }
    }

    pub fn set(&mut self, index: usize, value: f32) {
        self.pairs[index] = (index, value);
    }

    pub fn get(&mut self, index: usize) -> (usize, f32) {
        self.pairs[index]
    }

    pub fn build(&mut self) {
        self.pairs.sort_by(|a, b| a.1.total_cmp(&b.1));
        for i in 0..self.length {
            self.map[self.pairs[i].0] = i;
        }
    }

    pub fn find_chain(&self, index: usize) -> Vec<usize> {
        let mut output: Vec<usize> = vec![];

        // start pair
        let start = self.pairs[index];

        // search left
        for i in (0..index).rev() {
            if (self.pairs[i].1 - start.1).abs() <= self.radius {
                output.push(self.pairs[i].0);
            } else {
                break;
            }
        }

        // search right
        for i in index + 1..self.length {
            if (self.pairs[i].1 - start.1).abs() <= self.radius {
                output.push(self.pairs[i].0);
            } else {
                break;
            }
        }

        output
    }
}
