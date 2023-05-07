use std::io;


fn main() {
    let mut board = vec![["*","*","*"],["*","*","*"],["*","*","*"]];
    let mut game_state = true;
    let mut current_user = true;
    for row in &board {
        if row.contains(&"*") {
            game_state = true;
        }
        else {
            game_state = false;
        }
    }
    let mut filled_cols: Vec<Vec<usize>> = Vec::new();
    while game_state  {
        board_printer(&board, &current_user);
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let filter_input: Vec<usize> = input.trim().split(" ").map(|x| x.parse().unwrap()).collect();
        if filled_cols.contains(&&filter_input) {
            println!("This block is filled already");
            continue;
        }
        if current_user == true {
            let row = &filter_input[0]-1;
            let col = &filter_input[1]-1;
            board[row][col] = "X";
        }
        else {
            let row = &filter_input[0]-1;
            let col = &filter_input[1]-1;
            board[row][col] = "O";
        }
        game_state = check_win(&board, &current_user);
        filled_cols.push(filter_input);
        current_user = !current_user;
    }
}


fn board_printer(board: &Vec<[&str;3]>,current_user: &bool) {
    println!("\t\t\t\t-------------");
    println!("\t\t\t\t| {} | {} | {} |",board[0][0],board[0][1],board[0][2]);
    println!("\t\t\t\t-------------");
    println!("\t\t\t\t| {} | {} | {} |",board[1][0],board[1][1],board[1][2]);
    println!("\t\t\t\t-------------");
    println!("\t\t\t\t| {} | {} | {} |",board[2][0],board[2][1],board[2][2]);
    println!("\t\t\t\t-------------");
    println!("Current Player: {}", if current_user == &true {"X"} else {"O"});
    println!("Type your move: (row<space>col)");
}

fn check_win(board: &Vec<[&str;3]>,current_user: &bool) -> bool {
    // First column of each row
    if board[0][0] == board[1][0] && board[0][0] == board[2][0] && board[0][0] != "*"  {
        board_printer(board, current_user);
        println!("Player with {} wins!",if current_user == &true {"X"} else {"O"});
        return false;
    }
    // L - R Diagonal (From up to down)
    else if board[0][0] == board[1][1] && board[0][0] == board[2][2] && board[0][0] != "*" {
        board_printer(board, current_user);
        println!("Player with {} wins!",if current_user == &true {"X"} else {"O"});
        return false;
    }
    // R - L Diagonal (From down to up)
    else if board[2][0] == board[1][1] && board[2][0] == board[0][2] && board[2][0] != "*"  {
        board_printer(board, current_user);
        println!("Player with {} wins!",if current_user == &true {"X"} else {"O"});
        return false;
    }
    // Second column of each row
    else if board[0][1] == board[1][1] && board[0][1] == board[2][1] && board[0][1] != "*"  {
        board_printer(board, current_user);
        println!("Player with {} wins!",if current_user == &true {"X"} else {"O"});
        return false;
    }
    // Third column of each row
    else if board[0][2] == board[1][2] && board[0][2] == board[2][2] && board[0][2] != "*"  {
        board_printer(board, current_user);
        println!("Player with {} wins!",if current_user == &true {"X"} else {"O"});
        return false;
    }
    // Upper row
    else if board[0][0] == board[0][1] && board[0][0] == board[0][2] && board[0][0] != "*"  {
        board_printer(board, current_user);
        println!("Player with {} wins!",if current_user == &true {"X"} else {"O"});
        return false;
    }
    // Center row
    else if board[1][0] == board[1][1] && board[1][0] == board[1][2] && board[1][0] != "*"  {
        board_printer(board, current_user);
        println!("Player with {} wins!",if current_user == &true {"X"} else {"O"});
        return false;
    }
    // Bottom row
    else if board[2][0] == board[2][1] && board[2][0] == board[2][2] && board[2][0] != "*"  {
        board_printer(board, current_user);
        println!("Player with {} wins!",if current_user == &true {"X"} else {"O"});
        return false;
    }
    else {
        return true;
    }
}