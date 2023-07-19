use std::io;
fn check_vertical(table: &Vec<[char;3]>, x: usize, y: usize) -> bool{
    if table[x][y] == 'o' || table[x][y] == 'x' {
        if table[x][y] == table[x+1][y] && table[x+1][y] == table[x+2][y] {
            true
        }
        else{
            false
        }
    }
    else{
        false
    }
}
fn check_horizontal(table: &Vec<[char;3]>, x:usize, y:usize) -> bool{
    if table[x][y] == 'o' || table[x][y] == 'x' {

        if table[x][y] == table[x][y+1] && table[x][y+1] == table[x][y+2] {
             true
        }
        else{
            false
        }
    }else{
        false
    }
}

fn print_table(table: &Vec<[char;3]>) -> bool{
    let mut verticality = false;
    let mut horizontality = false;
    for _x in 0..3{
            horizontality = horizontality || check_horizontal(table, _x, 0);
            for _y in 0..3{
                verticality = verticality || check_vertical(table, 0, _y);
                print!("    {}    ",table[_x][_y]);  
            }
            println!("");   
    }
    return horizontality || verticality;
}
fn checkdiagnol(table: &mut Vec<[char;3]>, character_to_check: char) -> bool {
    if character_to_check == table[1][1]{
        if (table[0][0] == table[1][1] && table[1][1] == table[2][2]) || (table[2][0] == table[1][1] && table[1][1] == table[0][2]){
            true
        }
        else{
            false
        }
    }
    else{
       false 
    }
}
fn main() {
    let mut table:Vec<[char; 3]>  = vec![['.', '.', '.'], ['.', '.', '.'], ['.', '.', '.']];
    println!("Let's start the game");
    loop{
        let valid = print_table(&table);
        if valid {
            println!("You won");
            return;
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
        table[x][y] = 'x';
        if checkdiagnol(&mut table, 'x') == true{
            println!("You won!");
            return;
        }

    }
}

