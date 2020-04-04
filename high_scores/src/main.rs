
fn main() {
    
    let arr: [usize; 4] = [1, 200, 3, 4];
    let slice = &arr[0..arr.len()];

    let high_scores = HighScores::new(&slice);


    let highest = high_scores.highest_score();
    let last = high_scores.last_added_score();

    println!("The highest score is {:?}", highest.unwrap());
    println!("The most recently added score is {:?}", last.unwrap());

}


pub struct HighScores<'a> {
    scores:  &'a [usize],
}

impl<'a> HighScores<'a> {

    pub fn new(scores: &'a &[usize]) -> Self {
        HighScores {
            scores,
        }
    }

    pub fn get_scores(&self) -> &[usize] {
        self.scores
    }

    pub fn last_added_score(&self) -> Option<usize> {
        match self.scores.last() {
            Some(s) => Some(*s),
            None => None,
        }
    }

    pub fn highest_score(&self) -> Option<usize> {
        match self.scores.iter().max() {
            Some(s) => Some(*s),
            None => None,
        }
    }

    
    pub fn top_three(&self) -> Vec<usize> {
        let mut top_scores = self.scores.to_vec();
        &top_scores.sort();
        &top_scores.reverse();
        if top_scores.len() > 3 {
            (&top_scores[0..3]).to_vec()
        }
        else{
            top_scores
        }
    }

}