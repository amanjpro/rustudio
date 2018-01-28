pub mod gap_buffer;

use gap_buffer::LineBuffer;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::io::{BufReader, BufRead, BufWriter, Write};


const MAXIMUM_GAP_SIZE: usize = 512;
#[derive(Debug)]
pub struct Buffer {
    file_name: Option<String>,
    is_saved: bool,
    buffer: Vec<LineBuffer>,
    gap_start: usize,
    gap_end: usize,
    len: usize
}

impl Buffer {
    pub fn with_capacity(capacity: usize) -> Self {
        Buffer {
            file_name: Option::None,
            is_saved: false,
            buffer: Buffer::fill(capacity),
            len: 0,
            gap_start: 0,
            gap_end: if capacity > 0 { capacity - 1 } else { capacity },
        }
    }

    pub fn new() -> Self {
        Buffer {
            file_name: Option::None,
            is_saved: false,
            buffer: Buffer::fill(MAXIMUM_GAP_SIZE),
            gap_start: 0,
            len: 0,
            gap_end: MAXIMUM_GAP_SIZE - 1,
        }
    }

    fn fill(size: usize) -> Vec<LineBuffer> {
        let mut index = 0;
        let mut vec = Vec::with_capacity(size);
        while index < size {
            vec.push(LineBuffer::default());
            index += 1;
        }
        vec
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn get_line_at(&self, idx: usize) -> Option<&LineBuffer> {
        if self.gap_start != 0 {
            if idx < self.gap_start {
                self.buffer.get(idx)
            } else {
                self.buffer.get(idx + self.gap_end - self.gap_start + 1)
            }
        } else { Option::None }
    }

    pub fn get_current_line_index(&self) -> Option<usize> {
        if self.gap_start > 0 {
            Option::Some(self.gap_start - 1)
        } else { Option::None }
    }

    pub fn get_current_line(&self) -> Option<&LineBuffer> {
        if self.gap_start > 0 {
            self.get_line_at(self.gap_start - 1)
        } else { Option::None }
    }

    pub fn get_mut_line_at(&mut self, idx: usize) -> Option<&mut LineBuffer> {
        if self.gap_start != 0 {
            if idx < self.gap_start {
                self.buffer.get_mut(idx)
            } else {
                self.buffer.get_mut(idx + self.gap_end - self.gap_start + 1)
            }
        } else { Option::None }
    }

    pub fn get_mut_current_line(&mut self) -> Option<&mut LineBuffer> {
        let gap_start = self.gap_start;
        if gap_start > 0 {
            self.get_mut_line_at(gap_start - 1)
        } else { Option::None }
    }

    pub fn count(&self) -> usize {
        self.len
    }

    pub fn get_cursor_index(&self) -> Option<(usize, usize)> {
        if let Some(line) = self.get_current_line() {
            if let Some(col) = line.get_current_index() {
                self.get_current_line_index().map(|row| { (row, col) })
            } else { Option::None }
        } else { Option::None }
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

    pub fn move_cursor(&mut self, row: usize, col: usize) {
        self.seek(row);
        if let Some(line) = self.get_mut_current_line() {
            line.seek(col);
        };
    }

    /// mutating buffers

    pub fn new_line(&mut self) {
        if self.gap_start == self.gap_end {
            let mut new_buffer = Buffer::fill(self.count() + MAXIMUM_GAP_SIZE);
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
        self.buffer[self.gap_start] = LineBuffer::new();
        self.len += 1;
        self.gap_start += 1;
        self.is_saved = false;
    }

    pub fn delete(&mut self) {
        if self.gap_start > 0 {
            self.gap_start -= 1;
            self.len -= 1;
            self.is_saved = false;
        }
    }

    pub fn put_char(&mut self, ch: char) {
        if ch == '\r' {
            return;
        } else if ch == '\n' {
            self.new_line();
        } else {
            if let Some(mut line) = self.get_mut_current_line() {
                line.insert(ch);
            };
        }
        self.is_saved = false;
    }

    pub fn apply_to_all(&self, f: &Fn(&LineBuffer) -> LineBuffer) -> Buffer {
        let mut transformed = Buffer::fill(self.buffer.capacity());
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

        Buffer {
            is_saved: false,
            file_name: self.file_name.clone(),
            buffer: transformed,
            len: self.len,
            gap_start: self.gap_start,
            gap_end: self.gap_end,
        }
    }

    pub fn for_each(&self, f: &mut FnMut(&LineBuffer) -> ()) {
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

    pub fn save_buffer(&mut self, path: &str) {
        let file = File::create(&Path::new(path)).expect(&format!("Cannot write to the path: {}", path));
        let mut writer = BufWriter::new(file);

        let new_line = &['\n' as u8];
        let line_num = self.count();
        let mut count = 0;
        self.for_each(&mut |line| {
            line.for_each(&mut |ch| { writer.write(&[*ch as u8]); });
            count += 1;
            if count < line_num {
                writer.write(new_line);
            }
        });
        self.is_saved = true;
        self.file_name = Option::Some(path.to_string());
    }

}

// // Public interface
// pub fn seek_to_line(buffer: &mut Buffer, row: usize) {
//     buffer.seek(row);
// }
//
// pub fn seek_to_col(buffer: &mut Buffer, row: usize, col: usize) {
//     seek_to_line(row);
//     if let Some(mut line) = get_mut_active_line(buffer) {
//         line.seek(col);
//     }
// }
//
// pub fn delete_line(buffer: &mut Buffer, row: usize) {
//     seek_to_line(row);
//     buffer.delete();
// }
//
// pub fn delete_char(buffer: &mut Buffer, row: usize, col: usize) {
//     seek_to_line(row);
//     if let Some(mut line) = get_mut_active_line(buffer) {
//         line.seek(col);
//         line.delete();
//     }
// }
//
// 
//
// pub fn empty_buffer() -> Buffer {
//     GapBuffer::new()
// }
//
// pub fn open_file(path: &str) -> Buffer {
//     let file = File::open(&Path::new(path)).expect(&format!("Cannot open the file: {}", path));
//     let reader = BufReader::new(file);
//     let lines = reader.lines();
//     let (lo, hi) = lines.size_hint();
//     println!("the size of the file is between {} and {:?}", lo, hi);
//     let size = hi.unwrap_or(lo);
//     let mut buffer: Buffer = GapBuffer::with_capacity(if size == 0 { 100 } else { size });
//     for line in lines {
//         if let Ok(line) = line {
//             for ch in line.chars() {
//                 put_char(&mut buffer, ch);
//             }
//             put_char(&mut buffer, '\n');
//         }
//     }
//     buffer
// }
//
// pub fn save_buffer(buffer: &mut Buffer, path: &str) {
//     let file = File::create(&Path::new(path)).expect(&format!("Cannot write to the path: {}", path));
//     let mut writer = BufWriter::new(file);
//
//     let new_line = &['\n' as u8];
//     let line_num = buffer.count();
//     let mut count = 0;
//     buffer.for_each(&mut |line| {
//         line.for_each(&mut |ch| { writer.write(&[*ch as u8]); });
//         count += 1;
//         if count < line_num {
//             writer.write(new_line);
//         }
//     });
// }
//
// pub fn get_active_line_index(buffer: &mut Buffer) -> usize {
//     let idx = buffer.get_caret();
//     idx
//     // if idx == 0 {
//     //     buffer.insert(LineBuffer::new());
//     //     idx
//     // } else { idx - 1 }
// }
//
// pub fn get_active_col_index(buffer: &mut Buffer) -> usize {
//     if let Some(line) = get_active_line(buffer) {
//         let idx = line.get_caret();
//         if idx == 0 { idx } else { idx - 1 }
//     } else { 0 }
// }
//
// pub fn get_line(buffer: &Buffer, l: usize) -> Option<&LineBuffer> {
//     buffer.get(l)
// }
//
// pub fn get_mut_line(buffer: &mut Buffer, l: usize) -> Option<&mut LineBuffer> {
//     buffer.get_mut(l)
// }
//
// pub fn get_char(buffer: &Buffer, l: usize, c: usize) -> Option<&char> {
//     if let Some(line) = buffer.get(l) {
//         line.get(c)
//     } else { None }
// }
//
// pub fn get_mut_char(buffer: &mut Buffer, l: usize, c: usize) -> Option<&mut char> {
//     if let Some(line) = buffer.get_mut(l) {
//         line.get_mut(c)
//     } else { None }
// }
//
// // Helper functions
//
// fn get_active_line(buffer: &mut Buffer) -> Option<&LineBuffer> {
//     let index = get_active_line_index(buffer);
//     buffer.get(index)
// }
//
// fn get_mut_active_line(buffer: &mut Buffer) -> Option<&mut LineBuffer> {
//     let index = get_active_line_index(buffer);
//     buffer.get_mut(index)
// }
//
//
//
// //
// // pub fn open(path: &str) -> Buffer {
// //     if let Ok(meta) = metadata(path) {
// //         if meta.is_file() {
// //             open_file(path) as usize)
// //         } else {
// //             open_file(path) // FIXME: this should list all the files
// //         }
// //     }
// // }
