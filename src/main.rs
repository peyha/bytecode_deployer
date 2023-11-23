use clap::Parser;
use std::cmp;
use hex;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    bytecode: String,
}

fn main() {
    let args = Cli::parse();

    //TODO check that the bytecode begins with 0x and is hexadecimal (0..9a..f)
    //TODO check that the bytecode has an even length

    let mut res: String = String::new();

    let n = args.bytecode.len();
    let n_chunk = (n-1) / 64;

    //TODO MSTORE8s for lastchunk
    
    for i_pos in 0..n_chunk{
        //PUSH N_BYTES TO STACK
        //number of bytes is number half of the size of the current chunk
        let n_bytes = (cmp::min((i_pos+1)*64, args.bytecode.len()) - i_pos*64) / 2;
        let push_instruction = 0x5F + n_bytes;
        res.push_str(format!("{:x?}", push_instruction).as_str());
        
        let debut = i_pos * 64;
        let fin = i_pos * 64 + 2 * n_bytes;
        res.push_str(&args.bytecode[debut..fin]);
        
        //PUSH1
        let push_instruction = 0x60;
        res.push_str(format!("{:x?}", push_instruction).as_str());

        //OFFSET
        let offset = 32 * i_pos;
        let offset_hexa = format!("{:x?}", offset);
        if(offset_hexa.len() == 1){
            res.push('0');
        }
        res.push_str(offset_hexa.as_str());
        //MSTORE
        let mstore_instruction = 0x52; 
        res.push_str(format!("{:x?}", mstore_instruction).as_str());
    }

    // MSTORE8 for the last bytes
    for i_pos in ((64*n_chunk)..n).step_by(2){
        // PUSH1
        let push_instruction = 0x60;
        res.push_str(format!("{:x?}", push_instruction).as_str());
        
        // Byte
        res.push_str(&args.bytecode[i_pos..(i_pos+2)]);

        // PUSH1
        let push_instruction = 0x60;
        res.push_str(format!("{:x?}", push_instruction).as_str());

        //OFFSET
        let offset = i_pos / 2;
        res.push_str(format!("{:x?}", offset).as_str());

        //MSTORE8
        let mstore_instruction = 0x53;
        res.push_str(format!("{:x?}", mstore_instruction).as_str());
        
    }    
    println!("{}", res);
}
