use dice::Die;
use rand::Rng;

struct LinearScanDie {
    probabilities: Vec<f64>
}

impl Die for LinearScanDie {
    fn from_probabilities(ps: Vec<f64>) -> LinearScanDie {
        assert!(ps.iter().sum::<f64>() == 1f64);
        LinearScanDie { probabilities: ps }
    }

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

#[cfg(test)]
mod tests {
    use test::Bencher;

    use rand::SeedableRng;
    use rand::XorShiftRng;

    use dice::linear_scan_die::LinearScanDie;
    use dice::Die;

    #[test]
    #[should_panic]
    fn test_make() {
        LinearScanDie::from_probabilities(vec![0.25, 0.25, 0.25]);
    }

    #[test]
    fn test_roll() {
        let mut rng: XorShiftRng = XorShiftRng::from_seed([1, 2, 3, 4]);
        let die = LinearScanDie::from_probabilities(vec![0.25, 0.25, 0.5]);
        print!("{:?}\n", die.histogram(&mut rng, 1000));
    }

    #[bench]
    fn bench_roll(bencher: &mut Bencher) {
        let mut rng: XorShiftRng = XorShiftRng::from_seed([1, 2, 3, 4]);
        let die = LinearScanDie::from_odds(vec![1; 128]);
        bencher.iter(|| die.roll(&mut rng))
    }
}
