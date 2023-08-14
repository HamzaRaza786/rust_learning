use std::io;

fn check_vertical(table: &Vec<[char; 3]>, x: usize, y: usize) -> bool {
    if table[x][y] == 'o' || table[x][y] == 'x' {
        if table[x][y] == table[x + 1][y] && table[x + 1][y] == table[x + 2][y] {
            true
        } else {
            false
        }
    } else {
        false
    }
}
fn check_horizontal(table: &Vec<[char; 3]>, x: usize, y: usize) -> bool {
    if table[x][y] == 'o' || table[x][y] == 'x' {
        if table[x][y] == table[x][y + 1] && table[x][y + 1] == table[x][y + 2] {
            true
        } else {
            false
        }
    } else {
        false
    }
}

fn print_table(table: &Vec<[char; 3]>) -> bool {
    let mut verticality = false;
    let mut horizontality = false;
    for _x in 0..3 {
        horizontality = horizontality || check_horizontal(table, _x, 0);
        for _y in 0..3 {
            verticality = verticality || check_vertical(table, 0, _y);
            print!("    {}    ", table[_x][_y]);
        }
        println!("");
    }
    return horizontality || verticality;
}
fn checkdiagnol(table: &mut Vec<[char; 3]>, character_to_check: char) -> bool {
    if character_to_check == table[1][1] {
        if (table[0][0] == table[1][1] && table[1][1] == table[2][2])
            || (table[2][0] == table[1][1] && table[1][1] == table[0][2])
        {
            true
        } else {
            false
        }
    } else {
        false
    }
}
fn award_winner(i: u8) {
    let player = if i % 2 == 0 { "Player 1" } else { "Player 2" };
    println!("{} won!", player);
}
fn get_coordinates() -> (usize, usize) {
    println!("Enter the coordinates you want to change x    y");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).unwrap();
    let mut iterat = guess.split_whitespace();
    let x = iterat.next().unwrap().parse::<usize>().unwrap_or(3);
    let y = iterat.next().unwrap().parse::<usize>().unwrap_or(3);
    return (x, y);
}
fn main() {
    let mut table: Vec<[char; 3]> = vec![['.', '.', '.'], ['.', '.', '.'], ['.', '.', '.']];
    println!("Let's start the game");
    let mut _i = 1;
    while _i < 10 {
        let valid = print_table(&table);
        if valid {
            award_winner(_i);
            return;
        }
        let (x, y) = get_coordinates();
        if x > 2 || y > 2 {
            println!("Kindly enter indexes in range");
            continue;
        }
        let character: char = if _i % 2 == 0 { 'x' } else { 'o' };
        if table[x][y] == '.' {
            table[x][y] = character
        } else {
            println!("Entered wrong coordinates");
            continue;
        };
        if checkdiagnol(&mut table, character) == true {
            award_winner(_i);
            return;
        }
        _i += 1;
    }
    println!("It's a draw :(");
}
