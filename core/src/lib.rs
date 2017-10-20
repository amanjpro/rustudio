pub mod gap_buffer;

use gap_buffer::GapBuffer;

type LineBuffer = GapBuffer<Vec<char>>;
type Buffer = GapBuffer<LineBuffer>;

