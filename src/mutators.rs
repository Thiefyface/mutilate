#![allow(unused_parens)]
#![allow(unused_imports)]
use std::io;

impl BitFlipper{
    pub fn new(inp:String ,inp_seed:usize,inp_output:Vec<u8>,tmp_vec_inp:Vec<u8>)->BitFlipper{
        return BitFlipper{  
                            input:inp,
                            seed:inp_seed,
                            output:inp_output,
                            tmp_vec:tmp_vec_inp,
                         }
    }

    pub fn mutate(&mut self) -> Option<Vec<u8>>{
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
         
        // 0,1 => bit0,byte0 
        // 1,2 => revert1, bit0,byte1 
    

        // ideally 
        // |byte| |byte| |byte|...
        // bit0,byte0, bit0,byte1... 
        // bit1,byte0, bit1,byte1...

        return Some(self.output.to_vec());
    } 

    pub fn set_seed(&mut self,new_seed: usize){
        self.seed = new_seed;
    }




    pub fn init_output(&mut self) -> &Vec<u8>{
        self.output = Vec::with_capacity(self.input.len()); 
        self.output.extend_from_slice(self.input[0..].as_bytes()); 
        return &self.output; 
    }

}

#[derive(Debug)]
pub struct BitFlipper{
    pub seed: usize,
    pub input: String,
    pub output: Vec<u8>,
    pub tmp_vec:Vec<u8>, 
}
