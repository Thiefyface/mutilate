mod mutators;
use std::path::Path;
use std::error::Error;
use std::fs::{self,File}; 
use std::io::{self,Write,Read};
use std::{env,thread,time};
use std::convert::TryInto;
use std::process::{exit, Command, Stdio};


fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() < 0x3{
        println!("[-_-]; Not enough args");
        println!("{}: <inp_file> <output>",args[0]);
        println!("--seed(usize)  => Which seed to start with");
        println!("--count(usize) => How many cases to output"); 
        return;
    }
    
    let inp_file = &args[1];
    let out_file = &args[2];
    let extra_args: Vec<String> = args[3..].to_vec();

    let mut seed:usize=0x0;
    let mut count:usize=0xFFFFFFFFFFFF;
    let mut maxlen=-1;
    let mut target_process: String="".to_string();
    let mut target_args: Vec<String> = Vec::with_capacity(10); 

    for a in 0..extra_args.len(){
        if extra_args[a] == "--seed" { 
            seed = extra_args[a+1].parse().unwrap();
        } 
        if extra_args[a] == "--count" {
            count = extra_args[a+1].parse().unwrap();
        } 
        if extra_args[a] == "--maxlen"{
            maxlen = extra_args[a+1].parse().unwrap();
        }
        if extra_args[a] == "--"{
            target_process = extra_args[a+1].to_string();
            target_args = extra_args[a+2..].to_vec();
        }
    }

        
    let inp =fs::read_to_string(inp_file).expect("unable to read input");

    let mut out: Vec<u8> = Vec::with_capacity(inp.len());
    let mut tmp_vec: Vec<u8> = vec![0;inp.len()];
    let stdout = io::stdout;
    let strlen = inp.len();

    let mut bitflip= mutators::BitFlipper::new(inp,seed,out,tmp_vec);
    bitflip.init_output();

    if target_process.len() > 0{
        println!("Process to spawn => {}",target_process);
        let mut cmd=Command::new(&target_process);
        for a in target_args{
            let cmd=cmd.arg(a);
        }

       if out_file == "@@"{
 
            for i in 0..count{
                let new_str:&Vec<u8> = &bitflip.mutate().unwrap();
                let run_cmd = match cmd.stdin(Stdio::piped())
                                       .stdout(Stdio::piped())
                                       .spawn(){
                    Err(why) => panic!("nogo on subproc: {}",target_process),
                    Ok(run_cmd) => run_cmd,
                };
                
                println!("run_cmd => {:?}",run_cmd);
                match run_cmd.stdin.unwrap().write_all(new_str){
                    Err(why) => panic!("no write to stdin: {}",
                                        why.description()),
                    Ok(_) => println!("sent to prog!"), 
                }

                //cmd_stdin.write_all(new_str); 
                //cmd_stdin.write_all(b"asdf");

                let mut s = String::new();
                match run_cmd.stdout.unwrap().read_to_string(&mut s){
                    Err(why) => panic!("couldn't read cmd stdout: {}",
                                            why.description()),
                    Ok(_) => println!("cmd out => {}",s),
                }

            }
        } else {
            for i in 0..count{
                {
                    let out_path=Path::new(out_file);
                    let mut file = match File::create(&out_path) {
                        Err(why) => panic!("nogo on thingy"), 
                        Ok(file) => file,
                    };
            
                    let new_str:&Vec<u8> = &bitflip.mutate().unwrap();
                    file.write_all(new_str); 
                }
                let asdf=cmd.output().expect("failed [;_;]");
                println!("output? {:?}",asdf);
            }
        }

    } else { 

        if out_file == "@@"{
            for i in 0..count{
                let new_str:&Vec<u8> = &bitflip.mutate().unwrap();
                io::stdout().write_all(new_str);    
            }
        } else {
            let out_path=Path::new(out_file);
            println!("writing to {}",out_file);
            let mut file = match File::create(&out_path) {
                Err(why) => panic!("nogo on thingy"), 
                Ok(file) => file,
            };
        
            for i in 0..count{
                let new_str:&Vec<u8> = &bitflip.mutate().unwrap();
                file.write_all(new_str); 
            }
        }
          
    }
}
