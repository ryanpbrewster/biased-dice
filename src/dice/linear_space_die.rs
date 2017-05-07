use dice::Die;
use rand::Rng;

struct LinearSpaceDie {
    probabilities: Vec<f64>
}

impl LinearSpaceDie {
    pub fn from_probabilities(ps: Vec<f64>) -> LinearSpaceDie {
        LinearSpaceDie { probabilities: ps }
    }
}

impl Die for LinearSpaceDie {
    fn probabilities(&self) -> &[f64] {
        self.probabilities.as_slice()
    }

    fn roll(&self, rng: &mut Rng) -> u32 {
        unimplemented!()
    }
}