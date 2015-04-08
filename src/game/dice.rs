use rand;
use rand::distributions::{IndependentSample, Range};

pub struct DieRoll {
    number: u32,
    size: u32,
    results: Vec<u32>,
}

impl DieRoll {
    fn roll_dice(&mut self) {
        let between = Range::new(1, self.size);
        let mut rng = rand::thread_rng();
        for _ in 0..self.number {
            let roll = between.ind_sample(&mut rng);
            self.results.push(roll as u32);
        }
    }
    pub fn new(number: u32, size: u32) -> Self {
        let mut roll = DieRoll {
            number: number,
            size: size,
            results: Vec::<u32>::new(),
        };
        roll.roll_dice();
        roll
    }
    pub fn reroll(&mut self) {
        self.results = Vec::<u32>::new();
        self.roll_dice();
    }
    pub fn total(&self) -> u32 {
        let mut total = 0u32;
        for die in self.results.iter() {
            total += die.clone();
        }
        total
    }
    pub fn hits(&self, tn: u32) -> u32 {
        let mut hits = 0u32;
        for die in self.results.iter() {
            if (die.clone() >= tn) {
                hits += 1u32;
            }
        }
        hits
    }
}
