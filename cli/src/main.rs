extern crate ncurses;
extern crate rustudio_buffer;
extern crate rustudio_engine;

use ncurses::*;
use rustudio_buffer::*;
use rustudio_engine::*;

fn curses_init() {
    initscr();
    raw();
    /* Allow for extended keyboard (like F1). */
    keypad(stdscr(), true);
    noecho();
    //noecho();               // Don't echo keystrokes
    cbreak();               // Disable line buffering
    keypad(stdscr(), true);   // Enable special keys to be recorded
}

fn main() {
    let str = "";
    //if argc > 1 {
    //  fn = argv[1];               // Set the filename
    //}

    curses_init();                  // Initialize ncurses
    let mut engine = Engine::new();
    let mut stay_alive = true;
    while stay_alive  {
        let input = getch() as u32;
        match std::char::from_u32(input) {
            Option::Some(ch) => {
                stay_alive = engine.input(ch);
            }
            Option::None     =>
                (),
        }
        refresh();
    }

    // let mut buffer = empty_buffer();
    // put_char(&mut buffer, 'T');
    // put_char(&mut buffer, 'e');
    // put_char(&mut buffer, 's');
    // put_char(&mut buffer, 't');
    // put_char(&mut buffer, '\n');
    // put_char(&mut buffer, '\n');
    // put_char(&mut buffer, '\n');
    // put_char(&mut buffer, 'T');
    // put_char(&mut buffer, 'e');
    // put_char(&mut buffer, 's');
    // put_char(&mut buffer, 't');
    // put_char(&mut buffer, '\n');
    // put_char(&mut buffer, '\n');
    // put_char(&mut buffer, '\n');
    // put_char(&mut buffer, '\n');
    // put_char(&mut buffer, 'T');
    // put_char(&mut buffer, 'e');
    // put_char(&mut buffer, 's');
    // put_char(&mut buffer, 't');
    // put_char(&mut buffer, '\n');
    // save_buffer(&mut buffer, "/Users/amanjsherwany/Desktop/test2-1.txt");
    //
    // seek_to_line(&mut buffer, 4);
    // put_char(&mut buffer, '3');
    // put_char(&mut buffer, '3');
    // seek_to_col(&mut buffer, 4, 0);
    // put_char(&mut buffer, '4');
    // seek_to_col(&mut buffer, 4, 5);
    // put_char(&mut buffer, '5');
    // seek_to_col(&mut buffer, 1, 0);
    // seek_to_col(&mut buffer, 1, 3);
    // put_char(&mut buffer, '5');
    //
    // save_buffer(&mut buffer, "/Users/amanjsherwany/Desktop/test2.txt");
    //
    // let mut buffer = open_file("/Users/amanjsherwany/Desktop/test.txt");
    //
    // save_buffer(&mut buffer, "/Users/amanjsherwany/Desktop/test3.txt");

    refresh();                      // Refresh display
    endwin();                       // End ncurses mode
}
