extern crate ncurses;
extern crate rustudio_buffer;

use ncurses::*;
use rustudio_buffer::*;

fn curses_init() {
    initscr();              // Start ncurses mode
    noecho();               // Don't echo keystrokes
    cbreak();               // Disable line buffering
    //keypad(stdscr(), true);   // Enable special keys to be recorded
}

fn main() {
    let str = "";
    // if argc > 1 {
    //     fn = argv[1];               // Set the filename
    // }

    curses_init();                  // Initialize ncurses

    refresh();                      // Refresh display
    endwin();                       // End ncurses mode
}
