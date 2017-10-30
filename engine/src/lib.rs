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

// pub fn end_word(buffer: &mut Buffer) -> Selection {
//     let start_line = get_active_line_index(buffer);
//     let start_col = get_current_col(buffer);
//
// }

// pub fn start_word(buffer: &mut Buffer) -> Selection {

// pub fn end_of_line(buffer: &mut Buffer) -> Selection {

// pub fn start_of_line(buffer: &mut Buffer) -> Selection {

// pub fn start_of_text(buffer: &mut Buffer) -> Selection {

// pub fn end_of_file(buffer: &mut Buffer) -> Selection {

// pub fn start_of_file(buffer: &mut Buffer) -> Selection {

pub fn down(buffer: &mut Buffer) -> Selection {
    let line = get_active_line_index(buffer);
    let col = get_active_col_index(buffer);
    Selection {
        start_line: line + 1,
        start_col: col,
        end_line: line,
        end_col: col,
    }
}

pub fn up(buffer: &mut Buffer) -> Selection {
    let line = get_active_line_index(buffer);
    let col = get_active_col_index(buffer);
    Selection {
        start_line: line,
        start_col: col,
        end_line: line - 1,
        end_col: col,
    }
}

pub fn left(buffer: &mut Buffer) -> Selection {
    let line = get_active_line_index(buffer);
    let col = get_active_col_index(buffer);
    Selection {
        start_line: line,
        start_col: col,
        end_line: line,
        end_col: col - 1,
    }
}

pub fn right(buffer: &mut Buffer) -> Selection {
    let line = get_active_line_index(buffer);
    let col = get_active_col_index(buffer);
    Selection {
        start_line: line,
        start_col: col,
        end_line: line,
        end_col: col + 1,
    }
}

//pub fn scrap(buffer: &mut Buffer, selection: Select) {
//}
