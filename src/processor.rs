use std::fs::File;
//use std::fs::PathExtensions;
use std::env;
use std::os;
use std::io::Read;
use std::iter::Iterator;

pub struct Processor {
    memory: [u8; 4096],
    pc: usize   
}

impl Processor {

    pub fn new() -> Processor {
        let mut processor = Processor {
            memory: [0; 4096],
            pc: 0x200    
        };
        processor
    }

    pub fn load_game(&mut self, filename: String) {
        let mut path = env::current_dir().unwrap();
        path.push(filename.trim());
        print!("{}", filename);
        println!("{}", path.display());
        let mut crawler = File::open(&path).unwrap();
        self.load_to_memory(&mut crawler);
    }

    fn load_to_memory(&mut self, crawler: &mut File) {
        let mut buffer = [0; 1];
        match crawler.read(&mut buffer) {
            Ok(value) => {
                self.memory[self.pc] = buffer[0];
                print!("{:02x} ", self.memory[self.pc]);
                if self.pc < 4095 {
                    self.pc += 1;
                    self.load_to_memory(crawler)
                }
            }
            Err(e)    => { self.pc = 0x200 }
        }
    }

    fn disassemble(&mut self) {
    
    }
}
