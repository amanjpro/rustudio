extern crate rustudio_buffer;

use rustudio_buffer::*;

pub struct Selection {
    start_line: usize,
    start_col: usize,
    end_line: usize,
    end_col: usize,
}

pub fn open_line_above(buffer: &mut Buffer) {
    let row = get_active_line_index(buffer);
    seek_to_col(buffer, row, 0);
    put_char(buffer, '\n')
}

pub fn open_line_below(buffer: &mut Buffer) {
    let row = get_active_line_index(buffer) + 1;
    seek_to_col(buffer, row, 0);
    put_char(buffer, '\n')
}

pub fn go_to_start(buffer: &mut Buffer, selection: &Selection) {
    seek_to_col(buffer, selection.start_line, selection.start_col);
}

pub fn go_to_end(buffer: &mut Buffer, selection: &Selection) {
    seek_to_col(buffer, selection.end_line, selection.end_col);
}
