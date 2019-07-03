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