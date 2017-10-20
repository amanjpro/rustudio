pub mod gap_buffer;

use gap_buffer::GapBuffer;

type LineBuffer = GapBuffer<char>;
type Buffer = GapBuffer<LineBuffer>;


pub fn seek_to_line(buffer: &mut Buffer, row: usize) {
    buffer.seek(row);
}

pub fn seek_to_col(buffer: &mut Buffer, row: usize, col: usize) {
    buffer.seek(row);
    if let Some(mut line) = buffer.buffer.get_mut(row) {
        line.seek(col);
    }
}

pub fn delete_line(buffer: &mut Buffer, row: usize) {
    buffer.seek(row);
    buffer.delete();
}

pub fn delete_char(buffer: &mut Buffer, row: usize, col: usize) {
    buffer.seek(row);
    if let Some(mut line) = buffer.buffer.get_mut(row) {
        line.seek(col);
        line.delete();
    }
}
