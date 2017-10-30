use std::default::Default;
use std::fmt::Debug;

const MAXIMUM_GAP_SIZE: usize = 512;

#[derive(Debug)]
pub struct GapBuffer<T> where T: Default + Debug {
    buffer: Vec<T>,
    gap_start: usize,
    gap_end: usize,
    len: usize
}

impl<T> Default for GapBuffer<T> where T: Default + Debug {
    fn default() -> Self {
        GapBuffer {
            buffer: GapBuffer::fill(0),
            gap_start: 0,
            gap_end: 0,
            len: 0,
        }
    }
}

impl <T>GapBuffer<T> where T: Default + Debug {
    pub fn with_capacity(capacity: usize) -> Self {
        GapBuffer {
            buffer: GapBuffer::fill(capacity),
            len: 0,
            gap_start: 0,
            gap_end: if capacity > 0 { capacity - 1 } else { capacity },
        }
    }

    pub fn new() -> Self {
        GapBuffer {
            buffer: GapBuffer::fill(MAXIMUM_GAP_SIZE),
            gap_start: 0,
            len: 0,
            gap_end: MAXIMUM_GAP_SIZE - 1,
        }
    }

    fn fill(size: usize) -> Vec<T> {
        let mut index = 0;
        let mut vec = Vec::with_capacity(size);
        while index < size {
            vec.push(T::default());
            index += 1;
        }
        vec
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn get_item_at(&self, idx: usize) -> Option<&T> {
        if idx < self.gap_start {
            self.buffer.get(idx)
        } else {
            self.buffer.get(idx + self.gap_end - self.gap_start + 1)
        }
    }

    pub fn get_mut_item_at(&mut self, idx: usize) -> Option<&mut T> {
        if idx < self.gap_start {
            self.buffer.get_mut(idx)
        } else {
            self.buffer.get_mut(idx + self.gap_end - self.gap_start + 1)
        }
    }

    pub fn get(&self, item: usize) -> Option<&T> {
        self.buffer.get(item)
    }

    pub fn get_mut(&mut self, item: usize) -> Option<&mut T> {
        self.buffer.get_mut(item)
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

    pub fn get_current_index(&self) -> usize {
        self.gap_start
    }

    pub fn insert(&mut self, item: T) {
        if self.gap_start == self.gap_end {
            let mut new_buffer: Vec<T> = GapBuffer::fill(self.count() + MAXIMUM_GAP_SIZE);
            let mut index = 0;
            let mut new_index = 0;
            while index < self.len {
                if index == self.gap_start - 1 {
                    new_index += MAXIMUM_GAP_SIZE + 1;
                    index += 1;
                } else {
                    if let Some(item) = self.buffer.pop() {
                        new_buffer[new_index] = item;
                    }
                    index += 1;
                    new_index += 1;
                }
            }
            self.buffer = new_buffer;
            self.gap_end = self.gap_start + MAXIMUM_GAP_SIZE;
        }
        self.buffer[self.gap_start] = item;
        self.len += 1;
        self.gap_start += 1;
    }

    pub fn delete(&mut self) {
        if self.gap_start > 0 {
            self.gap_start -= 1;
            self.len -= 1;
        }
    }

    pub fn map<V>(&self, f: &Fn(&T) -> V) -> GapBuffer<V> where V: Default + Debug  {
        let mut transformed: Vec<V> = GapBuffer::fill(self.buffer.capacity());
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

        GapBuffer {
            buffer: transformed,
            len: self.len,
            gap_start: self.gap_start,
            gap_end: self.gap_end,
        }
    }

    pub fn for_each(&self, f: &mut FnMut(&T) -> ()) {
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
