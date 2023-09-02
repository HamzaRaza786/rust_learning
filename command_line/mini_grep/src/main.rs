use std::env;
use std::process;
use mini_grep::Config;
use std::rc::Rc;
enum List {
    Cons(i32, Rc<List>),
    Nil,
}


fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    if let Err(e) = mini_grep::run(config){
        eprintln!("Application error: {e}");
        process::exit(1);
    }

}
