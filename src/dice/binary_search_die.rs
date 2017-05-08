use dice::Die;
use rand::Rng;

struct BinarySearchDie {
    probabilities: Vec<f64>,
    lower_bounds: Vec<f64>
}

impl Die for BinarySearchDie {
    fn from_probabilities(ps: Vec<f64>) -> BinarySearchDie {
        assert!(ps.iter().sum::<f64>() == 1f64);
        let lbs = ps.iter().scan(0f64, |acc, &p| {
            *acc = *acc + p;
            Some(*acc)
        }).collect::<Vec<f64>>();

        BinarySearchDie {
            probabilities: ps,
            lower_bounds: lbs
        }
    }

    fn probabilities(&self) -> &[f64] {
        self.probabilities.as_slice()
    }

    fn roll(&self, rng: &mut Rng) -> usize {
        let x = rng.next_f64();

        bsearch(self.lower_bounds.as_slice(), |&v| x < v)
            .expect("rng.next_f64() < 1 and probabilities.sum() == 1 so there should always be a solution")
    }
}

// Find the smallest index, i, such that pred(vs[i]) is true.
// Assumes that pred(vs[i]) && j > i => pred(vs[j]).
// return None if there is no such index.
fn bsearch<P>(vs: &[f64], pred: P) -> Option<usize>
    where P: Fn(&f64) -> bool {
    if vs.is_empty() {
        return None;
    }
    // Hold as an invariant that pred(vs[lo]) == false && pred(vs[hi]) == true
    let mut lo = 0usize;
    let mut hi = vs.len() - 1;
    if pred(&vs[lo]) {
        return Some(lo);
    }
    if !pred(&vs[hi]) {
        return None;
    }


    // Once hi == lo + 1, the answer is Some(hi)
    while hi - lo > 1 {
        let mid = (lo + hi) / 2;
        if pred(&vs[mid]) {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    Some(hi)
}

#[cfg(test)]
mod tests {
    use test::Bencher;
    use rand;

    use dice::binary_search_die::BinarySearchDie;
    use dice::Die;

    #[test]
    #[should_panic]
    fn test_make() {
        BinarySearchDie::from_probabilities(vec![0.25, 0.25, 0.25]);
    }

    #[test]
    fn test_roll() {
        let mut rng = rand::thread_rng();
        let die = BinarySearchDie::from_probabilities(vec![0.25, 0.25, 0.5]);
        print!("{:?}\n", die.histogram(&mut rng, 1000));
    }

    #[bench]
    fn bench_roll(bencher: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let die = BinarySearchDie::from_odds(vec![1; 128]);
        bencher.iter(|| die.roll(&mut rng))
    }
}

