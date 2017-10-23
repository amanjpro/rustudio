const MAXIMUM_GAP_SIZE: usize = 512;

pub struct GapBuffer<T> {
    buffer: Vec<T>,
    gap_start: usize,
    gap_end: usize,
    len: usize
}

impl <T>GapBuffer<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        GapBuffer {
            buffer: Vec::with_capacity(capacity),
            len: 0,
            gap_start: 0,
            gap_end: if capacity > 0 { capacity - 1 } else { capacity },
        }
    }

    pub fn new() -> Self {
        GapBuffer {
            buffer: Vec::with_capacity(MAXIMUM_GAP_SIZE),
            gap_start: 0,
            len: 0,
            gap_end: MAXIMUM_GAP_SIZE - 1,
        }
    }

    pub fn get(&self, item: usize) -> Option<&T> {
        self.buffer.get(item)
    }

    pub fn get_mut(&mut self, item: usize) -> Option<&mut T> {
        self.buffer.get_mut(item)
    }

    pub fn seek(&mut self, col: usize) {
        if col > self.buffer.len() {
            return;
        }
        if col < self.gap_start {
            let mut index = col;
            while self.gap_start > index {
                self.buffer.swap(index, self.gap_end);
                self.gap_start -= 1;
                self.gap_end -= 1;
                index += 1;
            }
        } else if col > self.gap_start && col < self.gap_end {
            self.gap_start = col;
        } else {
            let mut index = col;
            while self.gap_start < index {
                self.buffer.swap(self.gap_start, index);
                self.gap_start += 1;
                self.gap_end += 1;
                index -= 1;
            }
        }
    }

    pub fn get_current_index(&self) -> usize {
        self.gap_start
    }

    pub fn insert(&mut self, item: T) {
        if self.gap_start >= self.gap_end {
            let mut new_buffer = Vec::with_capacity(self.buffer.capacity() * 2);
            let mut index = 0;
            let mut new_index = 0;
            while index < self.buffer.len() {
                if index == self.gap_start {
                    new_index += MAXIMUM_GAP_SIZE;
                    index += 1;
                    self.gap_end = new_index;
                } else {
                    unsafe {
                        let ref item = self.buffer.get_unchecked(index);
                        new_buffer.insert(new_index, *item);
                    }
                    index += 1;
                    new_index += 1;
                }
            }
        }
        self.buffer.insert(self.gap_start, item);
        self.len += 1;
        self.gap_start += 1;
    }

    pub fn delete(&mut self) {
        if self.gap_start > 0 {
            self.gap_start -= 1;
            self.len -= 1;
        }
    }

    pub fn count(&self) -> usize {
        self.len
    }
}
