#![allow(unused_parens)]
#![allow(unused_imports)]
use std::io;

//////////////////////////////////////////////////////
pub trait Mutilate{
    fn mutate(&mut self) -> Option<Vec<u8>>;
    fn init_output(&mut self);
    fn is_enabled(&mut self)->bool;
}

//////////////////////////////////////////////////////

#[derive(Debug)]
pub struct ChaosFlipper{
    pub seed: usize,
    pub input: String,
    pub output: Vec<u8>,
    pub tmp_vec:Vec<u8>, 
    pub enabled: bool,
}

impl ChaosFlipper{
    fn set_seed(&mut self,seed:usize){
        self.seed = seed;
    }
}

impl Mutilate for ChaosFlipper{
    fn mutate(&mut self) -> Option<Vec<u8>>{
        let mut byte_index:usize;
        let mut bit_index:usize;
        let mut tmp_seed=self.seed;
        let mut strlen=self.output.len();
        let mut counter=0x0;

        // build up the vector.
        //println!("self_seed => {}, tmp_seed => {}, strlen => {} ",self.seed, tmp_seed, strlen);
        while tmp_seed <= self.seed{ // loop till wrap
            byte_index = tmp_seed%strlen; 
            bit_index =  tmp_seed/strlen;
            //println!("byte_ind:0x{:x}, bit_ind:0x{:x}",byte_index,bit_index);
            self.tmp_vec[byte_index]^=(bit_index+1) as u8;
            tmp_seed = tmp_seed - (strlen+counter); 
            counter+=1;
        } 
        self.seed+=1;

        //println!("[^_^] Reverting (0x{:x},0x{:x})",byte_index,bit_index);
        //println!("tmp vector: {:?}",self.tmp_vec);
        for i in 0..strlen{
            self.output[i]^=self.tmp_vec[i];  
        }    
         
        return Some(self.output.to_vec());
    } 

    fn init_output(&mut self){
        self.output = Vec::with_capacity(self.input.len()); 
        self.output.extend_from_slice(self.input[0..].as_bytes()); 
    }

    fn is_enabled(&mut self)->bool{ self.enabled }
}


//////////////////////////////////////////////////////////////////
pub struct Empty{pub input:String,}

impl Mutilate for Empty{
    fn mutate(&mut self)-> Option<Vec<u8>>{
        return Some(self.input.as_bytes().to_vec())
    }

    fn init_output(&mut self){
    }

    fn is_enabled(&mut self)->bool{ return false; }
}
//////////////////////////////////////////////////////////////////
pub struct Truncator{
    pub seed: usize,
    pub input: String,
    pub output: Vec<u8>,
    pub enabled: bool,
}

impl Truncator{
    pub fn set_seed(&mut self,seed:usize){  self.seed = seed; }
}

impl Mutilate for Truncator{
    fn mutate(&mut self)->Option<Vec<u8>>{
        // for every given seed, shorten from the end.
        // if we end up going past len of whole buffer, 
        // move in 1 byte from end and restart
        let mut tmp_seed = self.seed;
        let strlen = self.input.len();
        let mut end_index = strlen; // where we start truncating from.

        while tmp_seed >= strlen && end_index >= 0{
            tmp_seed-=end_index;
            end_index-=1;
        }

        if tmp_seed > self.seed || tmp_seed > end_index{ // underflow
            self.enabled = false;
            return Some(self.output.to_vec());
        }

        //println!("truncate {}...{}",strlen-end_index,strlen-tmp_seed);
        //println!("strlen {}, end_ind {}, tmp_seed {}",strlen,end_index,tmp_seed);

        self.output.clear();
        self.output.extend_from_slice(self.input[strlen-end_index..strlen-tmp_seed].as_bytes());
        self.seed+=1;
        return Some(self.output.to_vec());

    }

    fn init_output(&mut self){
        self.output = Vec::with_capacity(self.input.len());
        self.output.extend_from_slice(self.input.as_bytes());

    }

    fn is_enabled(&mut self)->bool{ self.enabled }

}

//////////////////////////

static SPECIALNUMS: [u64; 38 ] = [  0x0, 0x01, 0x10, 0x7F, 0x80, 0xFF, // ind < 0x5  => special processing.
                                        0x1000, 0x7FFF, 0x8000, 0xFFFF,
                                        0x100000, 0x7FFFFF, 0x800000, 0xFFFFFF,
                                        0x0100000, 0x7FFFFFFF, 0x80000000, 0xFFFFFFFF,
                                        0x10000000, 0x7FFFFFFFFF, 0x8000000000, 0xFFFFFFFFFF,
                                        0x1000000000, 0x7FFFFFFFFFFF, 0x800000000000, 0xFFFFFFFFFFFF,
                                        0x100000000000, 0x7FFFFFFFFFFFFF, 0x80000000000000, 0xFFFFFFFFFFFFFF,
                                        0x10000000000000, 0x7FFFFFFFFFFFFFFF, 0x8000000000000000, 0xFFFFFFFFFFFFFFFF,
                                        0x1000000000000000, 0x7FFFFFFFFFFFFFFF, 0x8000000000000000, 0xFFFFFFFFFFFFFFFF ];
                                         
                                         
pub struct LenCorruption{
    pub seed: usize,
    pub input: String,
    pub output: Vec<u8>,
    pub enabled: bool,
}

impl LenCorruption{
    pub fn set_seed(&mut self,seed:usize){  self.seed = seed; }
}

impl Mutilate for LenCorruption{
    fn mutate(&mut self) -> Option<Vec<u8>>{

        //println!("Entered LenCorruption::mutate");
        let mut tmp_seed = self.seed;
        let strlen = self.input.len();
        let dst_index = tmp_seed/strlen; // where to hit in the buffer.
        let mut corrupt_val = SPECIALNUMS[tmp_seed%SPECIALNUMS.len()]; // which num to use out of array. 
        let mut val_len = 0x1;

        if dst_index == strlen{
           self.enabled = false;
           return Some(self.output.to_vec()); 
        }
        
        self.output.clear();
        
        // do first iteration.
        self.output.extend(self.input[0..dst_index].as_bytes());
        if corrupt_val <= 0xFF{   // => 1 byte write 
            self.output.push(corrupt_val as u8);  
            println!("<corrupt_val 0x{:x}",corrupt_val);
        } else {
            while corrupt_val > 0x0{
                self.output.push((corrupt_val&0xFF)as u8);
                corrupt_val = corrupt_val >> 8;
                val_len+=1;
                println!(">corrupt_val 0x{:x}",corrupt_val);
            }
        }
   
        if dst_index+val_len < strlen{ 
            println!("appending {:?}",self.input[dst_index+val_len..].as_bytes());
            self.output.extend(self.input[dst_index+val_len..].as_bytes());
        }
        //println!("Exit LenCorruption::mutate");
        self.seed+=1;

        return Some(self.output.to_vec()); 
    }

    fn init_output(&mut self){
        self.output = Vec::with_capacity(self.input.len());
        self.output.extend_from_slice(self.input.as_bytes());
    }

    fn is_enabled(&mut self) ->bool{ self.enabled }
}

