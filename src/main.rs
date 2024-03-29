use std::path::Path;
use std::error::Error;
use std::fs::{self,File}; 
use std::io::{self,Write,Read};
use std::{env,cmp};
use std::process::{Command, Stdio};

mod mutators;
use mutators::Mutilate;

fn main() {
    println!("{}[^_^] Mutilation [^_^]\n----------------------{}",RED,CLEAR); 
    let args: Vec<String> = env::args().collect();
    if args.len() < 0x3{
        INFO("Not enough args");
        println!("{}Usage: mutilator <inp_file> <output> -- <prog_to_run> <prog_args...>",GREEN);
        println!("<output> => filename or '@@' for stdout/pipe"); 
        println!("--seed <usize>  => Which seed to start with");
        println!("--count <usize> => How many cases to output"); 
        println!("--maxlen <usize> => Max output size");
        println!("--mutator [mutator_name] => use specific mutation function (default => all)");
        println!("[mutator_name] => ['chaos','lencorrupt',truncator']");
        println!("{}",CLEAR);
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
    let mut mutator_choice: String="all".to_string();

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
        if extra_args[a] == "--mutator"{
            mutator_choice = extra_args[a+1].parse().unwrap();
        }
        if extra_args[a] == "--"{
            target_process = extra_args[a+1].to_string();
            target_args = extra_args[a+2..].to_vec();
        }
    }

    let inp =fs::read_to_string(inp_file).expect("unable to read input");

    let mut mutilator_list = gen_mutilator_list(&inp,mutator_choice,seed); 


    let stdout = io::stdout;
    if target_process.len() > 0{
        println!("{}Process to spawn => {}{}",CYAN,target_process,CLEAR);
        let mut cmd=Command::new(&target_process);
        for a in target_args{
            let cmd=cmd.arg(a);
        }

       if out_file == "@@"{
 
            for m in &mut mutilator_list[..]{
                println!("m=>max_count == 0x{:x}",m.max_count());
                for i in 0..cmp::min(count,m.max_count()){
                    
                    // put a max count, break on that
                    // such that we don't need to check inside mutate.  
                    let new_str:&Vec<u8> = &m.mutate().unwrap();


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
            }
        } else {

            let out_path=Path::new(out_file);
            let mut file = match File::create(&out_path) {
                Err(why) => panic!("nogo on thingy"), 
                Ok(file) => file,
            };

            for m in &mut mutilator_list[..]{
                println!("m=>max_count == 0x{:x}",m.max_count());
                for i in 0..cmp::min(count,m.max_count()){
                    let new_str:&Vec<u8> = &m.mutate().unwrap();
                    file.write_all(new_str); 
                }
                let asdf=cmd.output().expect("failed [;_;]");
                println!("output? {:?}",asdf);
            }
        }

    } else { 
        if out_file == "@@"{
            for m in &mut mutilator_list[..]{
                println!("m=>max_count == 0x{:x}",m.max_count());
                for i in 0..cmp::min(count,m.max_count()){
                    let new_str:&Vec<u8> = &m.mutate().unwrap();
                    io::stdout().write_all(new_str);    
                }
            }
        } else {
            let out_path=Path::new(out_file);
            println!("{}writing to {}{}",CYAN,out_file,CLEAR);
            let mut file = match File::create(&out_path) {
                Err(why) => panic!("nogo on thingy"), 
                Ok(file) => file,
            };
            
            for m in &mut mutilator_list[..]{
                println!("m=>max_count == 0x{:x}",m.max_count());
                for i in 0..cmp::min(count,m.max_count()){
                    let new_str:&Vec<u8> = &m.mutate().unwrap(); 

                    match file.write_all(new_str){
                        Err(why) => panic!("Could not do mutilates on file"),
                        Ok(_) => (), 
                    } 
                }
            }
        } 
    }
}

pub fn gen_mutilator_list(inp :&String,
                          mutator_choice: String, 
                          seed: usize )-> Vec<Box<dyn mutators::Mutilate>>{ 
    let strlen = inp.len();
    let mut mutilator_list: Vec<Box<dyn mutators::Mutilate>> = Vec::new();

    if mutator_choice.find("chaos") != None || mutator_choice == "all"{
        let inp_copy_chaos = String::from(&inp[0..strlen]); // ideally, inp/out should be shared.
        let mut out_chaos: Vec<u8> = Vec::with_capacity(inp.len());
        let mut tmp_vec: Vec<u8> = vec![0;inp.len()];
        let mut chaos_flipper = mutators::ChaosFlipper{input:inp_copy_chaos,
                                                                 seed:seed,
                                                                 output:out_chaos,
                                                                 tmp_vec:tmp_vec,
                                                                 max_count:0x0};
        chaos_flipper.init_output();
        mutilator_list.push(Box::new(chaos_flipper));
    }

    if mutator_choice.find("truncator")  != None || mutator_choice == "all"{
        let inp_copy_trunc = String::from(&inp[0..strlen]);
        let mut out_trunc: Vec<u8> = Vec::with_capacity(inp.len());

        let mut truncator = mutators::Truncator{input:inp_copy_trunc,
                                                seed:seed,
                                                output:out_trunc,
                                                max_count:0x0};
        truncator.init_output();
        mutilator_list.push(Box::new(truncator));
    }


    if mutator_choice.find("lencorrupt")  != None || mutator_choice == "all"{
        let inp_copy_lencor = String::from(&inp[0..strlen]);
        let mut out_lencor: Vec<u8> = Vec::with_capacity(inp.len());

        let mut len_corruption = mutators::LenCorruption{input:inp_copy_lencor,
                                                         seed:seed,
                                                         output:out_lencor,
                                                         max_count:0x0};
        len_corruption.init_output();
        mutilator_list.push(Box::new(len_corruption));
    }


    if mutator_choice.find("inversion")  != None || mutator_choice == "all"{
        let inp_copy_inv = String::from(&inp[0..strlen]);
        let mut out_inv: Vec<u8> = Vec::with_capacity(inp.len());

        let mut inversion = mutators::Inversion{input:inp_copy_inv,
                                                     seed:seed,
                                                     output:out_inv,
                                                     max_count:0x0};
        inversion.init_output();
        mutilator_list.push(Box::new(inversion));
    }


    if mutator_choice.find("tetris")  != None || mutator_choice == "all"{
        let inp_copy_tetris = String::from(&inp[0..strlen]);
        let mut out_tetris: Vec<u8> = Vec::with_capacity(inp.len());

        let mut tetris = mutators::Tetris{input:inp_copy_tetris,
                                          seed:seed,
                                          output:out_tetris,
                                          max_count:0x0};
        tetris.init_output();
        mutilator_list.push(Box::new(tetris));
    }

    return mutilator_list;
}



/////////////////////////////////////////////////////////////////////
pub fn WARN(inp:&str) { println!("{}[!.!] {}{}",YELLOW,inp,CLEAR); }
pub fn ERR(inp:&str)  { println!("{}[x.x] {}{}",RED,inp,CLEAR); }
pub fn GOOD(inp:&str) {println!("{}[^_^] {}{}",GREEN,inp,CLEAR); }
pub fn INFO(inp:&str) {println!("{}[?_?] {}{}",CYAN,inp,CLEAR); }

static RED:&'static str="\x1b[1;31m";
static ORANGE:&'static str="\x1b[91m";
static GREEN:&'static str="\x1b[92m";
static LIME:&'static str="\x1b[99m";
static YELLOW:&'static str="\x1b[1;93m";
static BLUE:&'static str="\x1b[1;94m";
static PURPLE:&'static str="\x1b[95m";
static CYAN:&'static str="\x1b[96m";
static CLEAR:&'static str="\x1b[00m";


