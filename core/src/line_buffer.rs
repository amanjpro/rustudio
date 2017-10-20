const MAXIMUM_GAP_SIZE: usize = 512;

pub struct LineGapBuffer {
    line: Vec<char>,
    gap_start: usize,
    gap_end: usize,
}

impl LineGapBuffer {
    pub fn new() -> Self {
        LineGapBuffer {
            line: Vec::with_capacity(MAXIMUM_GAP_SIZE),
            gap_start: 0,
            gap_end: MAXIMUM_GAP_SIZE,
        }
    }

    pub fn seek(&mut self, col: usize) {
        if col > self.line.len() {
            return;
        }
        if col < self.gap_start {
            let mut index = col;
            while self.gap_start > index {
                unsafe {
                    let &ch = self.line.get_unchecked(index);
                    self.line.insert(self.gap_end, ch);
                }
                self.gap_start -= 1;
                self.gap_end -= 1;
                index += 1;
            }
        } else if col > self.gap_start && col < self.gap_end {
            self.gap_start = col;
        } else {
            let mut index = col;
            while self.gap_start < index {
                unsafe {
                    let &ch = self.line.get_unchecked(index);
                    self.line.insert(self.gap_start, ch);
                }
                self.gap_start += 1;
                self.gap_end += 1;
                index -= 1;
            }
        }
    }

    pub fn insert(&mut self, ch: char) {
        if self.gap_start < self.gap_end {
            self.line.insert(self.gap_start, ch);
            self.gap_start += 1;
        } else {
            let mut new_line = Vec::with_capacity(self.line.capacity() * 2);
            let mut index = 0;
            let mut new_index = 0;
            while index < self.line.len() {
                if index == self.gap_start {
                    new_index += MAXIMUM_GAP_SIZE;
                    index += 1;
                    self.gap_end = new_index;
                } else {
                    unsafe {
                        let &ch = self.line.get_unchecked(index);
                        new_line.insert(new_index, ch);
                    }
                    index += 1;
                    new_index += 1;
                }
            }
        }
    }

    pub fn delete(&mut self) {
        if self.gap_start > 0 {
            self.gap_start -= 1;
        }
    }
}
