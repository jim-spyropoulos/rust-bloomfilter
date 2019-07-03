
use mylib::MyBloomFilter;
use std::time::{Duration,Instant};
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

extern crate time;
fn main() -> Result<()> {

           

        let now = Instant::now();

        let mut bloom = MyBloomFilter::create(10000000 as u64,0.00001 as f64);
        let file = File::open("./input/bloom_input_10m.txt").expect("error in opening bloom_input file");
        let mut hash: u64 = 0 as u64;

        BufReader::new(file).lines().for_each(|line| {
           
           match line{
                    Err(why) => panic!("{:?}", why),
                    Ok(string) => hash = string.parse::<u64>().unwrap()
                }
                bloom.clever_insert_bloom(hash); 
        });

       
        println!("{} milliseconds took to insert 10 000 000 hashes to bloom filters.",now.elapsed().as_millis());
    
         let mut counter = 0; //counts the errcors

         let file2 = File::open("./input/bloom_test_10m.txt").expect("error in opening bloom test file");
         
         BufReader::new(file2).lines().for_each(|line| {
           
           match line{
                    Err(why) => panic!("{:?}", why),
                    Ok(string) => { hash = string.parse::<u64>().unwrap();
                                    match bloom.check_bloom(hash as u64){
                                        true => counter = counter + 1 ,
                                        false => () 
                                    } 
                                }
                }
        });

   
   println!("Errors are {}",counter);
    
    

    Ok(())
}
