use rand::Rng;

pub trait Die {
    fn probabilities(&self) -> &[f64];
    fn roll(&self, rng: &mut Rng) -> usize;

    fn histogram(&self, rng: &mut Rng, n: usize) -> Vec<u32> {
        let mut hist = vec![0; self.probabilities().len()];
        for _ in 1..n {
            let r = self.roll(rng);
            hist[r] = hist[r] + 1;
        }
        hist
    }
}

pub mod linear_space_die;