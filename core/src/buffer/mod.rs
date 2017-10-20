use std::cmp::max;

const MAXIMUM_GAP_SIZE: usize = 511;


struct GapBuffer {
    cursor: usize,
    before: Vec<char>,
    gap: Vec<char>,
    after: Vec<char>,
}

impl GapBuffer {
    pub fn new() -> Self {
        GapBuffer {
            cursor: 0,
            before: Vec::new(),
            gap: Vec::with_capacity(MAXIMUM_GAP_SIZE),
            after: Vec::new(),
        }
    }

    pub fn seek(&mut self, row: usize, col: usize) {
        // these should be atomic
        let leading_lines = GapBuffer::number_of_lines(&self.before);

        if row <= leading_lines {
            let row_index = GapBuffer::index_of_line(&self.before, row);
            let col_index = row_index + col - 1;
            {
                let len = self.gap.len();
                let (fst, snd) = self.gap.split_at(max(col_index, len));
                self.before = Vec::new();
                self.before.extend(fst);
                let mut tmp = Vec::new();
                tmp.extend(snd);
                tmp.extend(&self.after);
                self.after = tmp;
            }
            self.cursor = 0;
            self.gap.clear();
            return;
        }

        let gap_lines = GapBuffer::number_of_lines(&self.gap);

        if row <= leading_lines + gap_lines {
            self.cursor = col;
            return
        }

        let row_index = GapBuffer::index_of_line(&self.after, row - leading_lines - gap_lines);
        let col_index = row_index + col - 1;
        {
            let len = self.gap.len();
            let (fst, snd) = self.gap.split_at(max(col_index, len));
            self.before.extend(fst);
            self.after = Vec::new();
            self.after.extend(snd);
        }
        self.cursor = 0;
        self.gap.clear();
        return;
    }

    pub fn insert(&mut self, ch: char) {
        // these should be atomic
        if self.cursor >= MAXIMUM_GAP_SIZE {
            {
                let len = self.gap.len();
                let (fst, snd) = self.gap.split_at(len / 2);
                self.before.extend(fst);
                let mut tmp = Vec::new();
                tmp.extend(snd);
                tmp.extend(&self.after);
                self.after = tmp;
            }
            self.cursor = 0;
            self.gap.clear();
        }

        self.gap.insert(self.cursor, ch);
        self.cursor = self.cursor + 1;
    }

    pub fn delete(&mut self) {
        if self.cursor > 0 {
            self.cursor = self.cursor - 1;
            self.gap.remove(self.cursor);
        } else {
            let indx = self.before.len() - 1;
            self.before.remove(indx);
        }
    }

    pub fn delete_at(&mut self, at: usize) {
        if at <= self.before.len() {
            self.before.remove(at);
        } else if at <= self.before.len() + self.gap.len() {
            self.gap.remove(at);
            if at <= self.cursor {
                self.cursor = self.cursor - 1;
            }
        } else {
            self.after.remove(at);
        }
    }

    fn index_of_line(vector: &Vec<char>, line_number: usize) -> usize {
        let mut count = 0;
        let mut prev_is_new_line: bool = false;
        let mut index = 0;

        if line_number == 0 {
            return 0
        }

        while index < vector.len() {
            if vector[index] == '\n' {
                prev_is_new_line = true;
                count = count + 1;
            } else {
                if vector[index] == '\r' && !prev_is_new_line {
                    count = count + 1;
                }
                prev_is_new_line = false;
            }
            if line_number == count {
                break;
            }
            index = index + 1;
        }
        index
    }

    fn number_of_lines(vector: &Vec<char>) -> usize {
        let mut count = 0;
        let mut prev_is_new_line: bool = false;
        for &ch in vector.iter() {
            if ch == '\n' {
                prev_is_new_line = true;
                count = count + 1;
            } else {
                if ch == '\r' && !prev_is_new_line {
                    count = count + 1;
                }
                prev_is_new_line = false;
            }
        }
        count
    }
}
