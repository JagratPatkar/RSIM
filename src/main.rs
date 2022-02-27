use std::fs::File;
use std::io::{BufReader,Read};
use std::io::Result;

struct InsMem{
    mem : Vec<u8>,
    path : String
}

impl InsMem{
    fn populate_mem(&mut self) -> Result<()> {
        let f = File::open(self.path.as_str())?;
        let reader = BufReader::new(f);
        let mut b_reader = reader.bytes();
        loop{
            let byte = b_reader.next();
            if let Some(c) = byte.as_ref(){
                if let Ok(b) = c { self.mem.push(*b); }
                else { break; } 
            } 
            else { break; } 
        }
        Ok(())
    }

    fn print_mem(&self){
        println!("{:?}",self.mem);
    }
}


fn main() -> Result<()> {
    println!("RSIM Configured!");
    let mut ins_mem = InsMem{
        mem : Vec::new(),
        path : String::from("src/output.bin")
    };
    println!("Reading the Instructions!");
    ins_mem.populate_mem()?;
    ins_mem.print_mem();
    Ok(())
}
