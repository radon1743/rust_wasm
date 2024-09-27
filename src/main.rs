use std::io;
use colored::Colorize;

fn show(board:& Vec<char>){
    
    for i in 0..9{
        if board[i] == '_'{
            print!("{} ",i); 
        }
        else {
            if board[i] == 'X'{
                print!("{} ",board[i].to_string().red().bold().blink());
            }
            else{
                print!("{} ",board[i].to_string().green().bold());
            }
             
        }
        if (i+1)%3 == 0{
            println!();     
        } 
    }

}

fn player_input(board:&mut Vec<char>,x_o : bool) -> bool{
    println!("Enter number");
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("no input");
    let player_input:i32 = line.trim().parse().expect("wrong input"); 
    
    if board[player_input as usize] == '_'{
        if x_o == false{
        board[player_input as usize] = 'O';
        return true;
        }
        else {
            board[player_input as usize] = 'X';
            return false;
        }
    }
    else {
        println!("Wrong input");
        return x_o;
    }
    
}

fn game_check(board:&Vec<char>) ->(String, bool){
    
    if board[0] == board[1] && board[1]== board[2] || board[0] == board[3] && board[3]== board[6] || board[0] == board[4] && board[4] == board[8]{
        if board[0]!= '_'{
            return (board[0].to_string(),false);
        }
        
    } 
    
    else if board[1] == board[4] && board[4] == board[7] || board[3] == board[4] && board[4] == board[5] || board[2] == board[4] && board[4] == board[6] {
        if board[4]!= '_'{
            return (board[4].to_string(),false);
        }
    }
       
    else if board[2] == board[5]&&board[5] == board[8] || board[6] == board[7]&&board[7] == board[8]  {
        if board[8]!= '_'{
            return (board[8].to_string(),false);
        }
    }
    
    if board.into_iter().filter(|i|i==&&'_').count() == 0{
        return ("None".to_string(),false);
    }
    return ('_'.to_string(),true);

}    
    
fn game_loop(players:i32){
    let mut board:Vec<char> = vec!['_';9];
    
    let mut game:bool = true;
    let mut player:bool = true;
    //show(&mut board, 0);
    let mut winner:String = "_".to_string();
    while game {
        show(&mut board);
        if players == 2{
            player = player_input(&mut board,player);
        }
        else{
            player = player_input(&mut board,player);
        }
        (winner,game) = game_check(&board);
    }
    
    println!("Winner is {}",winner);
    show(&mut board);
    println!();
    main();
}
fn main() {
    println!("Hello, gamers!");
    println!("Press 1 for Single-player");
    println!("Press 2 for Two players");
    
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("no input");
    let n:i32 = line.trim().parse().expect("Wrong input");
    game_loop(n);
}
