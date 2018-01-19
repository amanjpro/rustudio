extern crate rustudio_buffer;

use rustudio_buffer::*;
use std::process;

type KeyCombination = Vec<char>;

const Ctrl: char = 17 as char;
const Esc: char = 27 as char;

pub struct Configuration {
    open_line_above: Vec<KeyCombination>,
    open_line_below: Vec<KeyCombination>,
    go_to_start: Vec<KeyCombination>,
    go_to_end: Vec<KeyCombination>,
    go_to_left: Vec<KeyCombination>,
    go_to_down: Vec<KeyCombination>,
    go_to_up: Vec<KeyCombination>,
    go_to_right: Vec<KeyCombination>,
    go_to_normal_mode: Vec<KeyCombination>,
    insert_char_here: Vec<KeyCombination>,
    write_buffer: Vec<KeyCombination>,
    quit_editor: Vec<KeyCombination>,
}

pub fn default_configuration() -> Configuration {
    Configuration {
        open_line_above: vec![vec!['o']],
        open_line_below: vec![vec!['O']],
        go_to_start: vec![vec!['0']],
        go_to_end: vec![vec!['$']],
        go_to_left: vec![vec!['h']],
        go_to_down: vec![vec!['j']],
        go_to_up: vec![vec!['k']],
        go_to_right: vec![vec!['l']],
        go_to_normal_mode: vec![vec![Ctrl, '['], vec![Esc]],
        insert_char_here: vec![vec!['i']],
        write_buffer: vec![vec![':', 'w', '\n']],
        quit_editor: vec![vec![':', 'q', '\n']],
    }
}

pub enum Mode {
    Normal,
    Insert
}

pub struct Selection {
    start_line: usize,
    start_col: usize,
    end_line: usize,
    end_col: usize,
}

pub struct Engine {
    mode: Mode,
    buffer: Buffer,
    conf: Configuration,
    command_buffer: Vec<char>,
}

impl Engine where {
    pub fn new() -> Self {
        Engine {
            mode: Mode::Normal,
            buffer: empty_buffer(),
            conf: default_configuration(),
            command_buffer: Vec::new(),
        }
    }

    pub fn input(&mut self, ch: char) -> bool {
        let mut stay_alive: bool = true;
        match self.mode {
            Mode::Insert => {
              self.command_buffer.push(ch);
              if self.conf.go_to_normal_mode.contains(&self.command_buffer) {
                  self.switch_mode(Mode::Normal);
              } else {
                  let len = self.command_buffer.len() - 1;
                  self.command_buffer.truncate(len);
                  put_char(&mut self.buffer, ch);
              }
              println!("{:?}", self.command_buffer);
            }
            Mode::Normal => {
              self.command_buffer.push(ch);
              println!("{:?}", self.command_buffer);
              if self.conf.open_line_above.contains(&self.command_buffer) {
                  self.open_line_above();
              } else if self.conf.open_line_below.contains(&self.command_buffer) {
                  self.open_line_below();
              // else if ch == self.conf.go_to_start {
              //     self.go_to_end();
              } else if self.conf.go_to_left.contains(&self.command_buffer) {
                  self.left();
                  self.clear_command_buffer();
              } else if self.conf.go_to_right.contains(&self.command_buffer) {
                  self.right();
                  self.clear_command_buffer();
              } else if self.conf.go_to_up.contains(&self.command_buffer) {
                  self.up();
                  self.clear_command_buffer();
              } else if self.conf.go_to_down.contains(&self.command_buffer) {
                  self.down();
                  self.clear_command_buffer();
              } else if self.conf.insert_char_here.contains(&self.command_buffer) {
                  self.switch_mode(Mode::Insert);
              } else if self.conf.write_buffer.contains(&self.command_buffer) {
                  save_buffer(&mut self.buffer, "/Users/amanjsherwany/Desktop/test2-1.txt");
                  self.clear_command_buffer();
              } else if self.conf.quit_editor.contains(&self.command_buffer) {
                  stay_alive = false;
                  self.clear_command_buffer();
              }
          }
        }
        stay_alive
    }


    fn open_line_above(&mut self) {
        {
            self.switch_mode(Mode::Insert);
        }
        let buffer = &mut self.buffer;
        let row = get_active_line_index(buffer);
        seek_to_col(buffer, row, 0);
        put_char(buffer, '\n')
    }

    fn open_line_below(&mut self) {
        {
            self.switch_mode(Mode::Insert);
        }
        let buffer = &mut self.buffer;
        let row = get_active_line_index(buffer) + 1;
        seek_to_col(buffer, row, 0);
        put_char(buffer, '\n')
    }

    // fn go_to_start(&mut self) {
    //     let buffer = &mut self.buffer;
    //     seek_to_col(buffer, selection.start_line, selection.start_col);
    // }
    //
    // fn go_to_end(&mut self) {
    //     let buffer = &mut self.buffer;
    //     seek_to_col(buffer, selection.end_line, selection.end_col);
    // }

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

    fn down(&mut self) {
        let buffer = &mut self.buffer;
        let line = get_active_line_index(buffer);
        let col = get_active_col_index(buffer);
        seek_to_col(buffer, line + 1, col);
    }

    fn up(&mut self) {
        let buffer = &mut self.buffer;
        let line = get_active_line_index(buffer);
        let col = get_active_col_index(buffer);
        seek_to_col(buffer, line - 1, col);
    }

    fn left(&mut self) {
        let buffer = &mut self.buffer;
        let line = get_active_line_index(buffer);
        let col = get_active_col_index(buffer);
        seek_to_col(buffer, line, col);
    }

    fn right(&mut self) {
        let buffer = &mut self.buffer;
        let line = get_active_line_index(buffer);
        let col = get_active_col_index(buffer);
        seek_to_col(buffer, line, col);
    }

    fn switch_mode(&mut self, mode: Mode) {
        self.clear_command_buffer();
        self.mode = mode;
    }

    fn clear_command_buffer(&mut self) {
        self.command_buffer.clear();
    }

    //pub fn scrap(buffer: &mut Buffer, selection: Select) {
    //}

    // fn right(self: &mut Self) -> Selection {
    //     let buffer = &mut self.buffer;
    //     let line = get_active_line_index(buffer);
    //     let col = get_active_col_index(buffer);
    //     Selection {
    //         start_line: line,
    //         start_col: col,
    //         end_line: line,
    //         end_col: col + 1,
    //     }
    // }
}
