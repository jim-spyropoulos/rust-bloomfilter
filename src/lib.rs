//for c functions
extern crate libc;



use std::fs::File;
use std::io::{BufRead, BufReader, Result};

use std::default::Default;
use std::hash::Hasher;
use bit_vec::BitVec;


extern {
            //fn double_input(input: libc::c_ulonglong) -> libc::c_ulonglong;
            fn XXH64(num: *mut ::libc::c_void, length: libc::size_t, seed: libc::c_ulonglong) -> libc:: c_ulonglong;
         }

/// An implementation of the Fowler–Noll–Vo hash function.

pub struct FNVHasher{
    fnv_offset_basis: u64 ,
    fnv_prime: u64,
    state: u64
}

impl Default for FNVHasher {
    fn default() -> Self{
        FNVHasher{ //for 64-bit hashes
            fnv_offset_basis : 0xcbf29ce484222325,
            fnv_prime : 1099511628211,
            state : 0xcbf29ce484222325
        }
    }
}

impl FNVHasher {
    pub fn new() -> Self{
        FNVHasher{
            ..FNVHasher::default()
        }
    }
}

impl Hasher  for FNVHasher{
    fn finish(&self) -> u64 {
        self.state
    }

    fn write(&mut self, bytes: &[u8]){

        for element in bytes.iter() {
            //for every byte
            self.state = self.state.wrapping_mul(self.fnv_prime);
            self.state ^= *element as u64; // as &u32;
        }
     }
}
impl FNVHasher {
    pub fn reset_hasher(&mut self){
        self.state = self.fnv_offset_basis;
    }
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
    //error_prob : f64  //probability of errors
}

 pub fn clever_bloom_size(size_bloom : &f64) -> f64{
        let l2 = size_bloom.log2(); //f64
        let d_floor = size_bloom - (2.0 as f64).powf(l2.floor()); //previous lower integer
        let d_ceil =  (2.0 as f64).powf(l2.ceil()) - size_bloom ; //next integer
       // println!("Log2 of {} is {}.d_floor {} d_ceil {} ",size_bloom,l2,d_floor,d_ceil);
        let mut ret:f64 = 0.0;
        if (d_floor < d_ceil){
            ret = (2.0 as f64).powf(l2.floor())
        }
        else{
            ret = (2.0 as f64).powf(l2.ceil())
        }
        //println!("returning {}",ret );
        //println!("bitand 13 & 4 = {}",13&4);
        ret
 }

 

impl MyBloomFilter{
    
   
    pub fn create(elements:u64, error_prob: f64) -> Self{
        let mut size_bloom = - (elements as f64* error_prob.ln()) / (2.0 as f64).ln().powf(2.0 as f64) ;
        let s2 = clever_bloom_size(&size_bloom); //size of bloom filter as power of 2
        let num_hashes = ( s2/elements as f64 ) * (2.0 as f64).ln();
        println!("Creating a bloom filter with {} bits and {} hash functions",s2.ceil(),num_hashes.ceil());
        MyBloomFilter{
            bit_table : BitVec::from_elem(s2 as usize, false),
            hash_functions : num_hashes.ceil() as u32,
            size : s2 as u64,//size_bloom.ceil() as u64,
        }
    }

   
    // fn run_hash(&self, index: u32, number : u64) -> u64 {
    //     let mut h = FNVHasher::new();
    //     h.write(&number.to_be_bytes());
    //     match index {
    //         0  =>  h.finish() , // if we have the first hash aka FNV
    //         _  =>  h.finish() + index as u64 * mix_hash(&number) // Kirsch-Mitzenmacher-Optimization(https://stackoverflow.com/questions/11954086/which-hash-functions-to-use-in-a-bloom-filter)
    //     }
    // }
    // //for this implementation i use formula : MIX_HASH + i * XXHASH
    //  fn run_hash1(&self, index: u32, number : u64) -> u64 {
         
    //      extern {
    //         //fn double_input(input: libc::c_ulonglong) -> libc::c_ulonglong;
    //         fn XXH64(num: *mut ::libc::c_void, length: libc::size_t, seed: libc::c_ulonglong) -> libc:: c_ulonglong;
    //      }

    //     // let my_ref : *mut libc::c_void = &mut hash as *mut _  as *mut libc::c_void;
    //     // let output = unsafe { XXH64(my_ref,64,12332243241) };
    //     let mut num = number;
    //     let my_ref : *mut libc::c_void = &mut num as *mut _  as *mut libc::c_void;
    //     match index {
    //         0  =>  mix_hash(&number) , // if we have the first hash aka FNV
    //         _  =>  { let out  = unsafe { XXH64(my_ref,64,12333241) };
    //                  mix_hash(&number) + out.wrapping_mul(index as u64) //to avoid overflow wrapping mul
    //                } // Kirsch-Mitzenmacher-Optimization(https://stackoverflow.com/questions/11954086/which-hash-functions-to-use-in-a-bloom-filter)
    //     }
    // }
    
   
    pub fn clever_insert_bloom(&mut self, number : u64){
        let mixh :u64 = mix_hash(&number);
       
        let mut num = number;
        let my_ref : *mut libc::c_void = &mut num as *mut _  as *mut libc::c_void;
        let xx_hash  = unsafe { XXH64(my_ref,8,1994) };

         //to avoid constant multiplications 
        let mut h_value:u64 = mixh;

        self.bit_table.set((h_value & self.size-1) as usize, true);
        for elem in 1 .. self.hash_functions {
            h_value = h_value.wrapping_add(xx_hash);
            self.bit_table.set( (h_value & self.size-1) as usize,true);
            //self.bit_table.set( ( mixh.wrapping_add(xx_hash.wrapping_mul(elem as u64))  & self.size-1 )as usize, true);
        }
    }
    pub fn insert_bloom(&mut self, number : u64){
        
        // let mut h = FNVHasher::new();
        // h.write(&number.to_be_bytes());
        // let fnv_hash:u64 = h.finish();

       let mixh :u64 = mix_hash(&number);
       
        let mut num = number;
        let my_ref : *mut libc::c_void = &mut num as *mut _  as *mut libc::c_void;
        let xx_hash  = unsafe { XXH64(my_ref,8,1994) };

        //to avoid constant multiplications 
        let mut h_value:u64 = mixh;
        self.bit_table.set((h_value % self.size) as usize, true);
        for elem in 1 .. self.hash_functions {
            h_value = h_value.wrapping_add(xx_hash);
            
            self.bit_table.set( ( h_value % self.size )as usize, true);
        
        }
    }



    pub fn check_bloom(&self,number : u64) -> bool{
        let mut flag = true;

        // let mut h = FNVHasher::new();
        // h.write(&number.to_be_bytes());
        // let fnv_hash:u64 = h.finish();

        let mixh :u64 = mix_hash(&number);
     
        let mut num = number;
        let my_ref : *mut libc::c_void = &mut num as *mut _  as *mut libc::c_void;
        let xx_hash  = unsafe { XXH64(my_ref,8,1994) };

        for elem in 0 .. self.hash_functions {
            // if elem==0 {
            //      if self.bit_table.get(  (mixh % self.size ) as usize) == Some(false)  {
            //         flag = false;
            //         break;
            //      }
            // }
            // else{
                if self.bit_table.get(   (mixh.wrapping_add(xx_hash.wrapping_mul(elem as u64))  & self.size-1 )as usize) == Some(false)  {
                    flag = false;
                    break;
                    }
           // }
        //     if self.bit_table.get( (self.run_hash(elem,number) % self.size) as usize) == Some(false)  {
        //    // if self.bit_table.get( (self.pipeline_hash(elem,number) % self.size) as usize) == Some(false)  {
        //         flag = false;
        //         break;
        //     }
        }
        flag
    }
}


#[cfg(test)]
mod tests {
    use super::*;

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