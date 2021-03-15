use std::collections::VecDeque;
use std::str::Chars;

pub struct StrCached<'a> {
    data: Chars<'a>,
    cached: VecDeque<char>
}

impl<'a> StrCached<'a> {
    pub fn new(data: &'a str) -> Self {
        Self {
            data: data.chars(),
            cached: VecDeque::new()
        }
    }

    pub fn cur_char(&mut self) -> Option<char> {
        if self.cached.is_empty() {
            if let Some(new_char) = self.data.next() {
                self.cached.push_back(new_char);
                Some(new_char)
            } else {
                None
            }
        } else {
            Some(self.cached[0])
        }
    }

    pub fn peek_char(&mut self) -> Option<char> {
        assert!(!self.cached.is_empty());
        if self.cached.len() >= 2 {
            Some(self.cached[1])
        } else {
            if let Some(peeked) = self.data.next() {
                self.cached.push_back(peeked);
                Some(peeked)
            } else {
                None
            }
        }
    }

    pub fn peek_n(&mut self, n: usize) -> Option<char> {
        assert!(!self.cached.is_empty());
        while self.cached.len() < n {
            if let Some(peeked) = self.data.next() {
                self.cached.push_back(peeked);
            } else {
                break;
            }
        }
        if self.cached.len() >= n {
            Some(self.cached[n - 1])
        } else {
            None
        }
    }

    pub fn next_char(&mut self) {
        if self.cached.len() > 0 {
            let _ = self.cached.pop_front();
        } else {
            let _ = self.data.next();
        }
    }
}
