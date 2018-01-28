use std::default::Default;
use std::fmt::Debug;

const MAXIMUM_GAP_SIZE: usize = 512;

#[derive(Debug, Default)]
pub struct LineBuffer {
    buffer: Vec<char>,
    gap_start: usize,
    gap_end: usize,
    len: usize
}


impl LineBuffer {
    pub fn with_capacity(capacity: usize) -> Self {
        LineBuffer {
            buffer: LineBuffer::fill(capacity),
            len: 0,
            gap_start: 0,
            gap_end: if capacity > 0 { capacity - 1 } else { capacity },
        }
    }

    pub fn new() -> Self {
        LineBuffer {
            buffer: LineBuffer::fill(MAXIMUM_GAP_SIZE),
            gap_start: 0,
            len: 0,
            gap_end: MAXIMUM_GAP_SIZE - 1,
        }
    }

    fn fill(size: usize) -> Vec<char> {
        let mut index = 0;
        let mut vec = Vec::with_capacity(size);
        while index < size {
            vec.push(char::default());
            index += 1;
        }
        vec
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn get_char_at(&self, idx: usize) -> Option<&char> {
        if idx < self.gap_start {
            self.buffer.get(idx)
        } else {
            self.buffer.get(idx + self.gap_end - self.gap_start + 1)
        }
    }


    /*
       columns start from 0

       ---S-------A----
       seek(3) should lead to
       --S-------A-----
       seek(4)
       ----S-------A---
     */
    pub fn seek(&mut self, seek_to: usize) {
        let gap_size = self.gap_end - self.gap_start;

        let seek_to  = if seek_to > 0 && seek_to >= self.count() {
            self.count() - 1
        } else {
            seek_to
        };

        if seek_to < 0 {
            return;
        } else if seek_to <= self.gap_start {
            let (swap_start, swap_end) = (seek_to, self.gap_start - 1);
            let mut index = swap_end;
            while index >= swap_start {
                self.buffer.swap(index, self.gap_end);
                self.gap_start -= 1;
                self.gap_end -= 1;
                if index == 0 {
                    break;
                }
                index -= 1;
            }
        } else {
            let (swap_start, swap_end) = (self.gap_end + 1, self.gap_end + seek_to);
            let mut index = swap_start;
            while index <= swap_end {
                self.buffer.swap(index, self.gap_start);
                self.gap_start += 1;
                self.gap_end += 1;
                index += 1;
            }
        }
    }

    pub fn get_current_index(&self) -> Option<usize> {
        if self.gap_start <= 0 { Option::None }
        else { Option::Some(self.gap_start - 1) }
    }

    pub fn get_caret(&self) -> usize {
        self.gap_start
    }

    pub fn insert(&mut self, ch: char) {
        if self.gap_start == self.gap_end {
            let mut new_line_buffer: Vec<char> = LineBuffer::fill(self.count() + MAXIMUM_GAP_SIZE);
            let mut index = 0;
            let mut new_index = 0;
            while index < self.len {
                if index == self.gap_start - 1 {
                    new_index += MAXIMUM_GAP_SIZE + 1;
                    index += 1;
                } else {
                    if let Some(item) = self.buffer.pop() {
                        new_line_buffer[new_index] = item;
                    }
                    index += 1;
                    new_index += 1;
                }
            }
            self.buffer = new_line_buffer;
            self.gap_end = self.gap_start + MAXIMUM_GAP_SIZE;
        }
        self.buffer[self.gap_start] = ch;
        self.len += 1;
        self.gap_start += 1;
    }

    pub fn delete(&mut self) {
        if self.gap_start > 0 {
            self.gap_start -= 1;
            self.len -= 1;
        }
    }

    pub fn apply_to_all(&self, f: &Fn(&char) -> char) -> LineBuffer {
        let mut transformed: Vec<char> = LineBuffer::fill(self.buffer.capacity());
        let mut index = 0;
        let len = self.buffer.len();
        while index < len {
            if index == self.gap_start {
                index = self.gap_end;
            } else {
                unsafe {
                    let elem = self.buffer.get_unchecked(index);
                    let telem = f(elem);
                    transformed.insert(index, telem);
                }
            }
            index += 1;
        }

        LineBuffer {
            buffer: transformed,
            len: self.len,
            gap_start: self.gap_start,
            gap_end: self.gap_end,
        }
    }

    pub fn for_each(&self, f: &mut FnMut(&char) -> ()) {
        let mut index = 0;
        let len = self.buffer.len();
        while index < len {
            if index == self.gap_start {
                index = self.gap_end;
            } else {
                unsafe {
                    let mut elem = self.buffer.get_unchecked(index);
                    f(elem);
                }
            }
            index += 1;
        }
    }

    pub fn count(&self) -> usize {
        self.len
    }
}
