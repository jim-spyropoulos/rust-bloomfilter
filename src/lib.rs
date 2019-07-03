// BSD 2-Clause License

// Copyright (c) 2019, Dimitris Spyropoulos
// All rights reserved.

// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:

// 1. Redistributions of source code must retain the above copyright notice, this
//    list of conditions and the following disclaimer.

// 2. Redistributions in binary form must reproduce the above copyright notice,
//    this list of conditions and the following disclaimer in the documentation
//    and/or other materials provided with the distribution.

// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
// CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
// OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

extern crate libc;
use bit_vec::BitVec;



//Hash functions that were used in the bloom filter:
//XXH64 taken from https://github.com/Cyan4973/xxHash
//A simle, yet fast mix-hash function

extern {
            fn XXH64(num: *mut ::libc::c_void, length: libc::size_t, seed: libc::c_ulonglong) -> libc:: c_ulonglong;
}

//A simple hash mix function for u64
#[inline]
pub fn mix_hash(number : &u64) ->u64{
        let  hash = *number;
        (hash >> 7 ) ^ (hash >> 13 ) ^ (hash >> 21 ) ^ (hash >> 31 ) ^ (hash >> 38 ) ^ (hash >> 46 ) ^ (hash >> 56 ) ^ *number 
}

// Implementation for Bloom filter
pub struct MyBloomFilter{
    bit_table : BitVec,
    hash_functions: u32, //number of hash functions
    size : u64 ,  //size of bloom filter 
}
 // Make the size of bloom filter equal to the nearest power of 2.
 // So, the expensive modulo operation during insert, is replaced by a bitwise and with a bitmask(that equals to bloom_size - 1 )
 
 pub fn modify_bloom_size(size_bloom : &f64) -> f64{

        let l2 = size_bloom.log2(); //f64
        let d_floor = size_bloom - (2.0 as f64).powf(l2.floor()); //previous lower integer
        let d_ceil =  (2.0 as f64).powf(l2.ceil()) - size_bloom ; //next integer
        if d_floor < d_ceil{ 
            (2.0 as f64).powf(l2.floor())
        }
        else{
            (2.0 as f64).powf(l2.ceil())
        }
        
 }

 

impl MyBloomFilter{
    
   // This function, given the number of elements to index and the desired error probability constructs a bloom filter.
   // To do so, it uses the theroretical formulas. See: https://en.wikipedia.org/wiki/Bloom_filter
   // Num_hashes is the number of hash functions to be used.

    pub fn create(elements:u64, error_prob: f64) -> Self{
        let size_bloom = - (elements as f64* error_prob.ln()) / (2.0 as f64).ln().powf(2.0 as f64) ;
        let s2 = modify_bloom_size(&size_bloom); //size of bloom filter as power of 2
        let num_hashes = ( s2.ceil()/elements as f64 ) * (2.0 as f64).ln();
        println!("Creating a bloom filter with {} bits and {} hash functions",s2.ceil(),num_hashes.ceil());
        MyBloomFilter{
            bit_table : BitVec::from_elem(s2.ceil() as usize, false),
            hash_functions : num_hashes.ceil() as u32,
            size : s2.ceil() as u64
        }
    }

   // Function insert_bloom performs the insertions to the bloom filter, by triggering the bit pointed by the respective hash function.
   // Instead of calling consecutively the hash functions with pipelined input Kirsch-Mitzenmacher-Optimization is used.
   // See : Kirsch-Mitzenmacher-Optimization https://www.eecs.harvard.edu/~michaelm/postscripts/tr-02-05.pdf
   // For every element we produce the mix_hash and XXH64 hash keys using the respective functions.
   // XXH64 is externed from the C binary with zero overhead.

    pub fn insert_bloom(&mut self, number : u64){
        let mixh :u64 = mix_hash(&number);
       
        let mut num = number;
        let my_ref : *mut libc::c_void = &mut num as *mut _  as *mut libc::c_void;
        let xx_hash  = unsafe { XXH64(my_ref,8,14654564) };

         //to avoid constant multiplications 
        let mut h_value:u64 = mixh;

        self.bit_table.set((h_value & self.size-1) as usize, true);
        for _elem in 1 .. self.hash_functions {
            h_value = h_value.wrapping_add(xx_hash);
            self.bit_table.set( (h_value & self.size-1) as usize,true);
        }
    }
   
    // Check_bloom function checks the posibility of an element's existence in the set.
    // If at most 1 hash functions points to an unitialized bit check_bloom return false.
    // Else, the element belongs to the set possibly.

    pub fn check_bloom(&self,number : u64) -> bool{
        let mut flag = true;

     
        let mixh :u64 = mix_hash(&number);
     
        let mut num = number;
        let my_ref : *mut libc::c_void = &mut num as *mut _  as *mut libc::c_void;
        let xx_hash  = unsafe { XXH64(my_ref,8,14654564) };

        for elem in 0 .. self.hash_functions {
                if self.bit_table.get(   (mixh.wrapping_add(xx_hash.wrapping_mul(elem as u64))  & self.size-1 )as usize) == Some(false)  {
                    flag = false;
                    break;
                    }
  
        }
        flag
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    use std::fs::File;
    use std::io::{BufRead, BufReader};  
    #[test]
    //Read 10 000 000 numbers from a file and insert them in the bloom filter.
    //Then ask for them and assert that u take 10 000 000 yes.
    fn bloom_correctness(){
        

        let mut bloom = MyBloomFilter::create(10000000 as u64,0.00001 as f64);
        let file = File::open("./input/bloom_input_10m.txt").expect("error in opening bloom_input file");
        let mut hash: u64 = 0 as u64;

        BufReader::new(file).lines().for_each(|line| {
           
           match line{
                    Err(why) => panic!("{:?}", why),
                    Ok(string) => hash = string.parse::<u64>().unwrap()
                }
                bloom.insert_bloom(hash); 
        });

       
    
        let mut counter = 0; //counts the errcors

        let file2 = File::open("./input/bloom_input_10m.txt").expect("error in opening bloom test file");
         
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
        assert_eq!(counter, 10000000);
    }
    #[test]
    fn bloom_10_elements_size(){
        let bloom = MyBloomFilter::create(10 as u64,0.1 as f64);
        assert_eq!(bloom.size, 48);
    }
    #[test]
    fn bloom_10_elements_hashes(){
        let bloom = MyBloomFilter::create(10 as u64,0.1 as f64);
        assert_eq!(bloom.hash_functions,4);
    }

     #[test]
     fn bloom_100000_elements_size(){
        let bloom = MyBloomFilter::create(100000 as u64,0.01 as f64);
        assert_eq!(bloom.size,958506);
     }
     
    #[test]
     fn bloom_100000_elements_hashes(){
        let bloom = MyBloomFilter::create(100000 as u64,0.01 as f64);
        assert_eq!(bloom.hash_functions,7);
     }

    #[test]
     fn bloom_10_elements_hash7(){
        let mut bloom = MyBloomFilter::create(10 as u64,0.1 as f64);
        bloom.insert_bloom(7 as u64);
        assert!(bloom.check_bloom(7 as u64));
     }

     #[test]
     fn bloom_10_elements_hash87(){
        let mut bloom = MyBloomFilter::create(10 as u64,0.1 as f64);
        bloom.insert_bloom(7 as u64);
        assert_eq!(bloom.check_bloom(87 as u64),false);
     }

     


    

}
