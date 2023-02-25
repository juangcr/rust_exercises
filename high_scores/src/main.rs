fn main() {
    let expected = [];
    let high_scores = HighScores::new(&expected);
    println!("{:?}", high_scores.scores());
    println!("{:?}", high_scores.latest());
    println!("{:?}", high_scores.personal_best());
    println!("{:?}", high_scores.personal_top_three());
}

#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores{scores: scores.to_vec()}
    }

    pub fn scores(&self) -> &[u32] {
        self.scores.as_slice()
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        let mut sc = self.scores.clone();
        sc.sort();
        sc.last().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut sc = self.scores.clone();
        sc.sort();
        sc.reverse();
        if sc.len() < 3 {
            sc.to_vec()
        } else {
            sc[..3].to_vec()
        }
    }
}
