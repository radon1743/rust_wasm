use std::io;
//use csv;
use rand::Rng;

fn show(board:& Vec<i32>){
    println!();
    for i in 0..9{
        if board[i] == 0{
            print!("{} ",i); 
        }
        else {
            if board[i] == -1{
                print!("X ");
            }
            else{
                print!("O ")
            }
             
        }
        if (i+1)%3 == 0{
            println!();     
        } 
    }

}



fn game_check(board:&Vec<i32>) ->(String, bool){
    
    if board[0] == board[1] && board[1]== board[2] || board[0] == board[3] && board[3]== board[6] || board[0] == board[4] && board[4] == board[8]{
        if board[0]!= 0{
            return (board[0].to_string(),false);
        }
        
    } 
    
    else if board[1] == board[4] && board[4] == board[7] || board[3] == board[4] && board[4] == board[5] || board[2] == board[4] && board[4] == board[6] {
        if board[4]!= 0{
            return (board[4].to_string()+" is the winner",false);
        }
    }
       
    else if board[2] == board[5]&&board[5] == board[8] || board[6] == board[7]&&board[7] == board[8]  {
        if board[8]!= 0{
            return (board[8].to_string()+" is the winner",false);
        }
    }
    
    if board.into_iter().filter(|i|i==&&0).count() == 0{
        return ("The game is a draw".to_string(),false);
    }
    return ('_'.to_string(),true);

}    

fn player_input(board:&mut Vec<i32>,x_o : bool) -> (bool,i32){
    println!("Enter number");
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("no input");
    let player_input:i32 = line.trim().parse().expect("wrong input"); 
    
    if board[player_input as usize] == 0{
        if x_o == false{
        board[player_input as usize] = 1;
        return (true,player_input as i32);
        }
        else {
            board[player_input as usize] = -1;
            return (false,player_input as i32);
        }
    }
    else {
        println!("Wrong input");
        return (x_o,0);
    }
    
}  

#[derive(Debug)]
struct Record {
    board_states: Vec<Vec<i32>>,
    next_move: Vec<i32>,    
    
}

fn comp_move(board:&mut Vec<i32>,x_o : bool) -> (bool,i32) {
    let mut rng = rand::thread_rng();
    let step = true;
    
    while step==true{
        let n:usize = rng.gen_range(0..=8);
        
        if board[n as usize] == 0{
            
            if x_o == false{
                board[n] = 1;
                return (true,n as i32);
            }
            else {
                board[n] = -1;
                return (false,n as i32);
            }
            
        }
        else {
            continue;
        }
    }
    
        
    (false,0)
}   



fn game_loop(players:i32){
    let mut board:Vec<i32> = vec![0;9];
    
    let mut game:bool = true;
    let mut player:bool = true;
    //show(&mut board, 0);
    let mut n:i32;
    let mut winner:String = "_".to_string();
    let mut rcd:Record = Record {board_states:vec![vec![0;9]],next_move:vec![0;9]};
    while game {
        show(&mut board);
        
        if players == 2{
            (player,n) = player_input(&mut board,player);
        }
        else{
            (player,n) = comp_move(&mut board,player);
           // player = player_input(&mut board,player);
        }
        rcd.next_move.push(n);
        let board2 = board.clone();
        rcd.board_states.push(board2);
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
