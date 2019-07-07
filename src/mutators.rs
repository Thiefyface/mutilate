#![allow(unused_parens)]
#![allow(unused_imports)]
use std::{io,cmp};

//////////////////////////////////////////////////////
pub trait Mutilate{
    fn mutate(&mut self) -> Option<Vec<u8>>;
    fn init_output(&mut self);
    fn max_count(&mut self) -> usize;
}

//////////////////////////////////////////////////////

#[derive(Debug)]
pub struct ChaosFlipper{
    pub seed: usize,
    pub input: String,
    pub output: Vec<u8>,
    pub tmp_vec:Vec<u8>, 
    pub max_count: usize,
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
        println!("Entered ChaosFlipper::mutate");
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
        self.max_count = 0x1000 // I guess? not sure. Can't have too high or else it will never stop.
    }


    fn max_count(&mut self) -> usize { self.max_count }
}


//////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct Truncator{
    pub seed: usize,
    pub input: String,
    pub output: Vec<u8>,
    pub max_count: usize,
}

impl Truncator{
    pub fn set_seed(&mut self,seed:usize){  self.seed = seed; }
}

impl Mutilate for Truncator{
    fn mutate(&mut self)->Option<Vec<u8>>{
        // for every given seed, shorten from the end.
        // if we end up going past len of whole buffer, 
        // move in 1 byte from end and restart
        //println!("Entered Truncator::mutate (seed: 0x{:x})",self.seed);
        let mut tmp_seed = self.seed;
        let strlen = self.input.len();

        let end_index = strlen - (tmp_seed / strlen); // => how many bytes to move inwards from right.
        let start_index = tmp_seed % end_index;        // => how many bytes to move inwards from left.  


        //println!("truncate {}...{}",start_index,end_index);
        //println!("strlen {}, end_ind {}, tmp_seed {}, max_count {}",strlen,end_index,tmp_seed,self.max_count);

        self.output.clear();
        self.output.extend_from_slice(self.input[start_index..end_index].as_bytes());
        //println!("output: {:?}", self.output);
        self.seed+=1;
        return Some(self.output.to_vec());

    }

    fn init_output(&mut self){
        self.output = Vec::with_capacity(self.input.len());
        self.output.extend_from_slice(self.input.as_bytes());
        let mut tmp_num : u128 = 0x0;
        
        for i in 1..(self.input.len()+1){
            tmp_num+=i as u128;  // calculate max num of iterations.        
            //println!("tmp_num += {} => {} ",i,tmp_num);
        } 
        if tmp_num > 0xFFFFFFFF{
            self.max_count = 0x10000;
        } else {
            self.max_count = tmp_num as usize;
        }
    }

    fn max_count(&mut self) -> usize { self.max_count }

}

//////////////////////////

static SPECIALNUMS: [u64; 22 ] = [  0x0, 0x01, 0x10, 0x7F, 0x80, 0xFF, // ind < 0x5  => special processing.
                                        0x1000, 0x7FFF, 0x8000, 0xFFFF,
                                        0x100000, 0x7FFFFF, 0x800000, 0xFFFFFF,
                                        0x0100000, 0x7FFFFFFF, 0x80000000, 0xFFFFFFFF,
                                        0x100000000, 0x7FFFFFFFF, 0x800000000, 0xFFFFFFFFF ];
                                         
                                         
#[derive(Debug)]
pub struct LenCorruption{
    pub seed: usize,
    pub input: String,
    pub output: Vec<u8>,
    pub max_count: usize,
}

impl LenCorruption{
    pub fn set_seed(&mut self,seed:usize){  self.seed = seed; }
}

impl Mutilate for LenCorruption{
    fn mutate(&mut self) -> Option<Vec<u8>>{

        println!("Entered LenCorruption::mutate");
        let mut tmp_seed = self.seed;
        let strlen = self.input.len();
        let dst_index = tmp_seed/strlen; // where to hit in the buffer.
        let mut corrupt_val = SPECIALNUMS[tmp_seed%SPECIALNUMS.len()]; // which num to use out of array. 
        let mut val_len = 0x1;

        self.output.clear();
        
        // do first iteration.
        self.output.extend(self.input[0..dst_index].as_bytes());
        if corrupt_val <= 0xFF{   // => 1 byte write 
            self.output.push(corrupt_val as u8);  
            //println!("<corrupt_val 0x{:x}",corrupt_val);
        } else {
            while corrupt_val > 0x0{
                self.output.push((corrupt_val&0xFF)as u8);
                corrupt_val = corrupt_val >> 8;
                val_len+=1;
                //println!(">corrupt_val 0x{:x}",corrupt_val);
            }
        }
   
        if dst_index+val_len < strlen{ 
            //println!("appending {:?}",self.input[dst_index+val_len..].as_bytes());
            self.output.extend(self.input[dst_index+val_len..].as_bytes());
        }
        //println!("Exit LenCorruption::mutate");
        self.seed+=1;

        return Some(self.output.to_vec()); 
    }

    fn init_output(&mut self){
        self.output = Vec::with_capacity(self.input.len());
        self.output.extend_from_slice(self.input.as_bytes());
        self.max_count = cmp::min(SPECIALNUMS.len() * self.input.len(),0xFFFFFFFF);
    }

    fn max_count(&mut self) -> usize { self.max_count }
}

//////////////////////////

#[derive(Debug)]
pub struct Inversion{
    pub seed: usize,
    pub input: String,
    pub output: Vec<u8>,
    pub max_count: usize,
}

impl Inversion{
    pub fn set_seed(&mut self,seed:usize){  self.seed = seed; }
}

impl Mutilate for Inversion{

    fn mutate(&mut self) -> Option<Vec<u8>>{
        //println!("hit inversion!");
        self.output[self.seed] = ((self.output[self.seed]^0xFF)+1);
        self.seed+=1;
        self.output[self.seed] = ((self.output[self.seed]^0xFF)+1);
        return Some(self.output.to_vec());
    }

    fn init_output(&mut self){
        self.output = Vec::with_capacity(self.input.len());
        self.output.extend_from_slice(self.input.as_bytes());
        self.output[self.seed] = ((self.output[self.seed]^0xFF)+1);
        self.max_count = self.input.len()-1;
    } 
    
    fn max_count(&mut self) -> usize { self.max_count }
}

////////////////////////
