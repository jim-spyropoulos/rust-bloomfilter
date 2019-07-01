
use mylib::MyBloomFilter;
use std::time::Instant;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
   
        let now = Instant::now();

        let mut bloom = MyBloomFilter::create(10000000 as u64,0.1 as f64);
        let file = File::open("/Users/jJimo/Desktop/bloom_input_10m.txt").expect("error in opening bloom_input file");
        let mut hash: u64 = 0 as u64;

        BufReader::new(file).lines().for_each(|line| {
           
           match line{
                    Err(why) => panic!("{:?}", why),
                    Ok(string) => hash = string.parse::<u64>().unwrap()
                }
                bloom.clever_insert_bloom(hash); 
        });

        // for line in BufReader::new(file).lines() {
        //         match line{
        //             Err(why) => panic!("{:?}", why),
        //             Ok(string) => hash = string.parse::<u64>().unwrap()
        //         }
        //         bloom.clever_insert_bloom(hash);
        // }
        println!("{} milliseconds took to insert 10 000 000 hashes to bloom filters.",now.elapsed().as_millis());
   
    let mut counter = 0; //counts the errors

    let file2 = File::open("/Users/jJimo/Desktop/bloom_test_10m.txt").expect("error in opening bloom test file");
    for line in BufReader::new(file2).lines() {
            match line{
                Err(why) => panic!("{:?}", why),
                Ok(string) =>{ hash = string.parse::<u64>().unwrap();
                               match bloom.check_bloom(hash as u64){
                                     true => counter = counter + 1 ,
                                     false => () //println!("{} definitely not in the set.",hash)
                                } 
                         }
            }
           

    }
   println!("Errors are {}",counter);
    
    

    Ok(())
}
 

// fn murmur3_32(seed: u32, number: u32) -> u32 {
//     const C1: u32 = 0xcc9e2d51;
//     const C2: u32 = 0x1b873593;
//     const R1: u32 = 15;
//     const R2: u32 = 13;
//     const M: u32 = 0x5;
//     const N: u32 = 0xe6546b64;

//     let mut k = number; //LittleEndian::read_u32(&hash);
//     k = k.wrapping_mul(C1).rotate_left(R1).wrapping_mul(C2);

//     let mut ret_hash = seed;
//     ret_hash ^= k;
//     ret_hash = ret_hash.rotate_left(R2);
//     ret_hash = ret_hash.wrapping_mul(M).wrapping_add(N);

//     //PHASE III
//     //ret_hash ^= 32;
//     //PHASE IV
//     ret_hash ^= ret_hash.wrapping_shr(R1);
//     ret_hash = ret_hash.wrapping_mul(0x85ebca6b);
//     ret_hash ^= ret_hash.wrapping_shr(R2);
//     ret_hash = ret_hash.wrapping_mul(0xc2b2ae35);
//     ret_hash ^= ret_hash.wrapping_shr(R1);

//     ret_hash
// }


//     //let mut bytes = hash.to_le_bytes();
    //     // for byte in bytes.iter(){
    //     //  print!("{} -",byte);
    //     // }
    //      let ret_hash = Murmur3_32(0,hash);
    //     print!("{} - {} - {} \n",hash,ret_hash,ret_hash % 2000000);

    //     //Insert to bloom filter
    //     bv.set(usize::try_from(ret_hash % 2000000).unwrap(),true);

    //    // hash = line.parse::<u32>().unwrap();

    // }
    // let mut h = FNVHasher::new();
    // let num :u32 = 72;
    // // i hash 
    // h.write(&num.to_be_bytes());
    // println!("FnvHasher finish {}",h.finish() % 15);
    // h.reset_hasher(); //then i reset 
    
    // h.write(&num.to_be_bytes());
    // println!("FnvHasher finish {}",h.finish() % 15);

    // println!("{}", FNV_32(0x48) % 15);