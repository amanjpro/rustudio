pub mod gap_buffer;

use gap_buffer::GapBuffer;
use std::fs::File;
use std::io::{BufReader, BufRead};

type LineBuffer = GapBuffer<char>;
type Buffer = GapBuffer<LineBuffer>;


// Public interface
pub fn seek_to_line(buffer: &mut Buffer, row: usize) {
    buffer.seek(row);
}

pub fn seek_to_col(buffer: &mut Buffer, row: usize, col: usize) {
    buffer.seek(row);
    if let Some(mut line) = get_mut_line(buffer) {
        line.seek(col);
    }
}

pub fn delete_line(buffer: &mut Buffer, row: usize) {
    buffer.seek(row);
    buffer.delete();
}

pub fn delete_char(buffer: &mut Buffer, row: usize, col: usize) {
    buffer.seek(row);
    if let Some(mut line) = get_mut_line(buffer) {
        line.seek(col);
        line.delete();
    }
}

pub fn put_char(buffer: &mut Buffer, ch: char) {
    if ch == '\r' {
        return;
    } else if ch == '\n' {
        buffer.insert(GapBuffer::new());
    } else {
        if let Some(mut line) = get_mut_line(buffer) {
            line.insert(ch);
        };
    }
}

fn open_file(path: &str) -> Buffer {
    let reader = BufReader::new(File::open(path).expect("file not found"));
    let lines = reader.lines();
    let (lo, hi) = lines.size_hint();
    let size = hi.unwrap_or(lo);
    let mut buffer: Buffer = GapBuffer::with_capacity(size);
    for line in lines {
        if let Ok(line) = line {
            for ch in line.chars() {
                put_char(&mut buffer, ch);
            }
            put_char(&mut buffer, '\n');
        }
    }
    buffer
}

// Helper functions

fn get_active_line_index(buffer: &Buffer) -> usize {
    let idx = buffer.get_current_index();
    if idx == 0 { idx } else { idx - 1 }
}

fn get_line(buffer: &Buffer) -> Option<&LineBuffer> {
    buffer.get(get_active_line_index(buffer))
}

fn get_mut_line(buffer: &mut Buffer) -> Option<&mut LineBuffer> {
    let idx = get_active_line_index(buffer);
    buffer.get_mut(idx)
}




//
// pub fn open(path: &str) -> Buffer {
//     if let Ok(meta) = metadata(path) {
//         if meta.is_file() {
//             open_file(path) as usize)
//         } else {
//             open_file(path) // FIXME: this should list all the files
//         }
//     }
// }
