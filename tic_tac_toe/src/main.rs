use std::io;
fn main() {
    let mut table  = vec![[".", ".", "."], [".", ".", "."], [".", ".", "."]];
     println!("Let's start the game");
    loop{ 
        for _x in 0..3{
            for _y in 0..3{
                print!("    {}    ",table[_x][_y]);  
            }
            println!("");
        }
         println!("Enter the coordinates you want to change x    y");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();
        let mut iterat = guess.split_whitespace();
        let x = iterat.next().unwrap().parse::<usize>().unwrap();
        let y = iterat.next().unwrap().parse::<usize>().unwrap();
        if x > 2 || y > 2 {
            println!("Kindly enter indexes in range");
            continue;
        }
        table[x][y] = "x";

    }
}

