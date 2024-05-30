fn main() {
    let index: [&str; 9] = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    gameloop(index);
}

const p1: &str = "x";
const p2: &str = "o";

fn gameloop(mut index: [&str; 9]) {
    let mut xes_turn: bool = true;
    let mut not_game_over: bool = true;

    while not_game_over {
        draw_board(index);
        
        let current_turn: &str;
        if xes_turn {current_turn = p1} else {current_turn = p2}
        println!("{current_turn} has to make a turn: ");

        let input: usize = get_input();
        let x: usize = input - 1;

        if check_if_field_is_empty(x ,index) {
            index[x] = current_turn;
            not_game_over = !check_for_win(index);
            if not_game_over {xes_turn = !xes_turn;} else {draw_board(index);}
        }

    }

    let current_turn: &str;
    if xes_turn {current_turn = p1} else {current_turn = p2}
    println!("\n{current_turn} has Won!");
}
    
fn check_if_field_is_empty(x:usize, index: [&str; 9]) -> bool {

    if index[x] != p1 && index[x] != p2 {
        return true;
    }
    return false
}

fn check_for_win(i: [&str; 9]) -> bool {
    
    if 
        i[0] == i[1] && i[1] == i[2] ||
        i[3] == i[4] && i[4] == i[5] ||
        i[6] == i[7] && i[7] == i[8] ||
        i[0] == i[3] && i[3] == i[6] ||
        i[1] == i[4] && i[4] == i[7] ||
        i[2] == i[5] && i[5] == i[8] ||
        i[0] == i[4] && i[4] == i[8] ||
        i[2] == i[4] && i[4] == i[6]
    {
        return true;
    }

    return false;
}

fn draw_board(
    index: [&str; 9],
) {
    let mut counter = 0;
    
    for _ in 1..=3 {
        
        for _ in 1..=3 {
            
            let r = index[counter];
            
            print!("|\t{r}\t");
            
            counter += 1;
        }
        
        print!("\n|---------------|---------------|---------------\n")
    }
    
}

pub fn get_input() -> usize{
 
    let mut input = String::new();
    let string = std::io::stdin().read_line(&mut input).expect("Failed to read line");
    match input.trim().parse::<usize>() {
        Ok(number) => {
            return number;
        }
        Err(_) => {
            print!("Error");
            return usize::MAX;
        }
    }
}