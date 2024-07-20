use std::collections::VecDeque;

#[derive(Debug)]
pub struct DataString(VecDeque<bool>);

impl DataString {
    pub fn new(data: Vec<bool>) -> Self {
        Self(VecDeque::from(data))
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn delete_leftmost(&mut self) {
        self.0.pop_front();
    }

    /// You must check for emptiness before calling this
    pub unsafe fn peek_leftmost(&self) -> Option<bool> {
        unsafe { self.0.front().copied().unwrap_unchecked() }
    }

    pub fn push_rightmost(&mut self, c: bool) {
        self.0.push_back(c)
    }
}
