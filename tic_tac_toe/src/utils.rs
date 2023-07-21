use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;


fn preview(table: &Vec<[char;3]>){
    for _x in 0..3{
        println!("({_x}, 0) ({_x}, 1) ({_x},2)");
        for _y in 0..3{
            print!("{}      ",table[_x][_y]);  
        }
        println!("");   
    }
}
pub fn key_strokes(table: &Vec<[char;3]>) {
    let stdin = stdin();
    //setting up stdout and going into raw mode 

    //detecting keydown events
    for c in stdin.keys() {
        //clearing the screen and going to top left corner     

        //i reckon this speaks for itself
        match c.unwrap() {
            Key::Ctrl('p') => preview(table),
            Key::Ctrl('x') => break,
            _ => print!("Hello"),
        }
 
    }
}

