pub mod gap_buffer;

use gap_buffer::GapBuffer;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::io::{BufReader, BufRead, BufWriter, Write};

type LineBuffer = GapBuffer<char>;
pub type Buffer = GapBuffer<LineBuffer>;


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

pub fn empty_buffer() -> Buffer {
    GapBuffer::new()
}

pub fn open_file(path: &str) -> Buffer {
    let file = File::open(&Path::new(path)).expect(&format!("Cannot open the file: {}", path));
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let (lo, hi) = lines.size_hint();
    println!("the size of the file is between {} and {:?}", lo, hi);
    let size = hi.unwrap_or(lo);
    let mut buffer: Buffer = GapBuffer::with_capacity(if size == 0 { 100 } else { size });
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

pub fn save_buffer(buffer: &mut Buffer, path: &str) {
    let file = File::create(&Path::new(path)).expect(&format!("Cannot write to the path: {}", path));
    let mut writer = BufWriter::new(file);

    let new_line = &['\n' as u8];
    let line_num = buffer.count();
    let mut count = 0;
    buffer.for_each(&mut |line| {
        line.for_each(&mut |ch| { writer.write(&[*ch as u8]); });
        count += 1;
        if count < line_num {
            writer.write(new_line);
        }
    });
}

pub fn get_active_line_index(buffer: &mut Buffer) -> usize {
    let idx = buffer.get_current_index();
    if idx == 0 {
        buffer.insert(LineBuffer::new());
        idx
    } else { idx - 1 }
}

pub fn get_line(buffer: &mut Buffer) -> Option<&LineBuffer> {
    let index = get_active_line_index(buffer);
    buffer.get(index)
}

pub fn get_mut_line(buffer: &mut Buffer) -> Option<&mut LineBuffer> {
    let index = get_active_line_index(buffer);
    buffer.get_mut(index)
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
