use dice::Die;
use std::iter::Sum;
use rand;
use rand::Rng;

struct LinearSpaceDie {
    probabilities: Vec<f64>
}

impl LinearSpaceDie {
    pub fn from_probabilities(ps: Vec<f64>) -> LinearSpaceDie {
        assert!(ps.iter().sum::<f64>() == 1f64);
        LinearSpaceDie { probabilities: ps }
    }
}

impl Die for LinearSpaceDie {
    fn probabilities(&self) -> &[f64] {
        self.probabilities.as_slice()
    }

    fn roll(&self, rng: &mut Rng) -> usize {
        let x = rng.next_f64();

        let mut acc = 0f64;
        for (i, &p) in self.probabilities.iter().enumerate() {
            acc += p;
            if acc >= x {
                return i;
            }
        }
        self.probabilities.len()
    }
}

#[test]
#[should_panic]
fn test_make() {
    LinearSpaceDie::from_probabilities(vec![0.25, 0.25, 0.25]);
}

#[test]
fn test_roll() {
    let mut rng = rand::thread_rng();
    let die = LinearSpaceDie::from_probabilities(vec![0.25, 0.25, 0.5]);
    print!("{:?}\n", die.histogram(&mut rng, 1000));
}
