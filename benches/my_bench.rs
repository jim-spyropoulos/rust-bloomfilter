extern crate criterion;

use criterion::Criterion;
use criterion::*;

use std::fs::File;
use std::io::{BufRead, BufReader};
use mylib::*;





fn bench_crit(c: &mut Criterion) {
        c.bench("formula1", Benchmark::new("custom_bench1",|b| b.iter(|| { 
        let mut bloom = MyBloomFilter::create(10000000 as u64,0.00001 as f64);
        let file = File::open("./input/bloom_input_10m.txt").expect("error in opening bloom_input file");
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