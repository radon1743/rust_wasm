use std::{path::Path,error::Error, io, process,fs::OpenOptions};
use csv::WriterBuilder;
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



fn game_check(board:&Vec<i32>) ->i32{
    
    if board[0] == board[1] && board[1]== board[2] || board[0] == board[3] && board[3]== board[6] || board[0] == board[4] && board[4] == board[8]{
        if board[0]!= 0{
            return board[0];
        }
        
    } 
    
    else if board[1] == board[4] && board[4] == board[7] || board[3] == board[4] && board[4] == board[5] || board[2] == board[4] && board[4] == board[6] {
        if board[4]!= 0{
            return board[4];
        }
    }
       
    else if board[2] == board[5]&&board[5] == board[8] || board[6] == board[7]&&board[7] == board[8]  {
        if board[8]!= 0{
            return board[8];
        }
    }
    
    if board.into_iter().filter(|i|i==&&0).count() == 0{
        return 0;
    }
    return 2;

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
    x_next_move: Vec<i32>,    
    o_next_move: Vec<i32>,
}

fn rand_move(board:&mut Vec<i32>,x_o : bool) -> (bool,i32) {
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

fn write_csv(rcd:Record,n:i32) -> Result<(), Box<dyn Error>>{
    let file_path = "record.csv";
    let file_exists = Path::new(file_path).exists();

    // Open the file in append mode if it exists, or create it otherwise
    let file = OpenOptions::new()
        .append(true)  // Open in append mode
        .create(true)  // Create the file if it doesn't exist
        .open(file_path)?;
    let mut wtr = WriterBuilder::new().has_headers(file_exists).from_writer(file);
    if !file_exists {
        wtr.write_record(&["board_states", "next_move"])?;
    }
    if n == 0 {
            for i in 0..rcd.board_states.len(){
                if i%2 == 0{
                    let next_move_str_x = format!("{:?}", rcd.x_next_move[i/2]);
                    let board_state_str = format!("{:?}", rcd.board_states[i]); 
                    wtr.write_record(&[board_state_str, next_move_str_x])?;    
                }
                else {
                    let board_state_str = format!("{:?}", rcd.board_states[i]); 
                    let next_move_str_o = format!("{:?}", rcd.o_next_move[(i-1) / 2]);
                    wtr.write_record(&[board_state_str, next_move_str_o])?; 
                }            
            }
            
            
        }
    else if n == -1{
           for i in 0..rcd.x_next_move.len(){
                let next_move_str_x = format!("{:?}", rcd.x_next_move[i]);
                let board_state_str = format!("{:?}", rcd.board_states[i*2]); 
                wtr.write_record(&[board_state_str, next_move_str_x])?;
           }
        }
        
        
    else if n == 1{
            for i in 0..rcd.o_next_move.len(){
                let next_move_str_o = format!("{:?}", rcd.o_next_move[i]);
                let board_state_str = format!("{:?}", rcd.board_states[(i*2)+1]); 
                wtr.write_record(&[board_state_str, next_move_str_o])?;
           }
        }
    
    Ok(())
    
}

fn game_loop(players:i32){
    let mut board:Vec<i32> = vec![0;9];
    
    let mut game= 2;
    let mut player:bool = true;
    //show(&mut board, 0);
    let mut n:i32;
   
    
    let mut rcd:Record = Record {board_states:vec![vec![0;9]],x_next_move:vec![],o_next_move:vec![]};
    while game==2{
        show(&mut board);
        
        if players == 2{
            (player,n) = player_input(&mut board,player);
        }
        else{
            (player,n) = rand_move(&mut board,player);
           // player = player_input(&mut board,player);
        }
        if player == false{
                rcd.x_next_move.push(n);
                println!("x = {}",n);  
        }
        else {
                rcd.o_next_move.push(n);
                println!("o = {}",n);
        }
        
        
        let board2 = board.clone();
        rcd.board_states.push(board2);
        game = game_check(&board);
    }
    
    
    
    show(&mut board);
    println!();
    if let Err(err) = write_csv(rcd,game) {
        println!("error running write_csv: {}", err);
        process::exit(1);
    }
    main();
}


fn main() {
    println!("Hello, gamers!");
    println!("Press 1 for Random-players");
    println!("Press 2 for Two players");   
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("no input");
    let n:i32 = line.trim().parse().expect("Wrong input");
    game_loop(n);
}