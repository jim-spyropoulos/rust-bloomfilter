#[macro_use]
extern crate criterion;

use criterion::Criterion;
use criterion::*;
use criterion::black_box;
use std::time::Duration;

use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use mylib::*;



// fn fibonacci(n: u64) -> u64 {
//     match n {
//         0 => 1,
//         1 => 1,
//         n => fibonacci(n-1) + fibonacci(n-2),
//     }
// }


// fn criterion_benchmark(c: &mut Criterion) {
//     c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
// }




fn bench_crit(c: &mut Criterion) {
        //c.measurement_time(Duration::new(100,0));
        c.bench("formyla1", Benchmark::new("mitsps",|b| b.iter(|| { 
        let mut bloom = MyBloomFilter::create(10000000 as u64,0.00001 as f64);
        let file = File::open("/Users/jJimo/Desktop/bloom_input_1m.txt").expect("error in opening bloom_input file");
        let mut hash: u64;

       for line in BufReader::new(file).lines() {
                    match line{
                        Err(why) => panic!("{:?}", why),
                        Ok(string) => hash = string.parse::<u64>().unwrap()
                    }
                    bloom.insert_bloom(hash);
                }
        })).sample_size(30));
}

criterion_group!(benches, bench_crit);
criterion_main!(benches);