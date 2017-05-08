use rand::Rng;

pub trait Die : Sized {
    fn from_probabilities(ps: Vec<f64>) -> Self;
    fn from_odds(odds: Vec<u32>) -> Self {
        let total: u32 = odds.iter().sum();
        let ps = odds.iter().map(|&v| v as f64 / total as f64).collect::<Vec<f64>>();
        Self::from_probabilities(ps)
    }

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

pub mod linear_scan_die;
pub mod binary_search_die;
