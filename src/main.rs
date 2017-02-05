use std::io;

use processor::Processor;

mod processor;

fn main() {
    let mut processor = Processor::new();    
    
    println!("Give a filename: ");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            
        }
        Err(e) => {
            println!("Error: {}", e)
        }
    }

    processor.load_game(input);

    //'main: loop {
        
            
        
    //}
    
}
