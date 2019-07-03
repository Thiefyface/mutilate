#![allow(unused_parens)]
#![allow(unused_imports)]
use std::io;

//////////////////////////////////////////////////////////////////
pub trait Mutilate{
    fn mutate(&mut self) -> Option<Vec<u8>>;
    fn init_output(&mut self);
    fn is_enabled(&mut self)->bool;
}

//////////////////////////////////////////////////////////////////
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
