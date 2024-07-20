use std::collections::VecDeque;

#[derive(Debug)]
pub struct ProgramString(VecDeque<bool>);

impl ProgramString {
    pub fn new(s: Vec<bool>) -> Self {
        Self(VecDeque::from(s))
    }

    pub fn next_cycling(&mut self) -> Option<bool> {
        match self.0.pop_front() {
            None => None,
            Some(b) => {
                self.0.push_back(b);
                Some(b)
            }
        }
    }
}
