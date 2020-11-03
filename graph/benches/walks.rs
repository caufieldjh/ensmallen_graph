#![feature(test)]
extern crate test;
use test::{Bencher, black_box};
extern crate graph;
use graph::test_utilities::*;

use graph::*;
use graph::test_utilities::{load_ppi, first_order_walker};
use rayon::iter::ParallelIterator;

#[bench]
fn bench_slow(b: &mut Bencher) {
    let mut graph = load_ppi(true, true, true, false, false, false).unwrap();
    let walker = first_order_walker(&graph).unwrap();
    
    b.iter(|| {
        for _ in 0..10 {
            black_box(
                graph.random_walks_iter(1, &walker).unwrap().collect::<Vec<Vec<NodeT>>>()
            );
        }
    });
}

#[bench]
fn bench_fast(b: &mut Bencher) {
    let mut graph = load_ppi(true, true, true, false, false, false).unwrap();
    let walker = first_order_walker(&graph).unwrap();

    graph.enable_fast_walk(true, true, None);
    
    b.iter(|| {
        for _ in 0..10 {
            black_box(
                graph.random_walks_iter(1, &walker).unwrap().collect::<Vec<Vec<NodeT>>>()
            );
        }
    });
}

fn bench_cache(b: &mut Bencher, level: f64) {
    let mut graph = load_ppi(true, true, true, false, false, false).unwrap();
    let walker = first_order_walker(&graph).unwrap();

    graph.enable_fast_walk(false, false, Some(level)).unwrap();
    
    b.iter(|| {
        for _ in 0..10 {
            black_box(
                graph.random_walks_iter(1, &walker).unwrap().collect::<Vec<Vec<NodeT>>>()
            );
        }
    });
}

#[bench]
fn bench_cache_05(b: &mut Bencher) {
    bench_cache(b, 0.05)
}

#[bench]
fn bench_cache_25(b: &mut Bencher) {
    bench_cache(b, 0.25)
}

#[bench]
fn bench_cache_50(b: &mut Bencher) {
    bench_cache(b, 0.5)
}

#[bench]
fn bench_cache_75(b: &mut Bencher) {
    bench_cache(b, 0.75)
}

#[bench]
fn bench_cache_95(b: &mut Bencher) {
    bench_cache(b, 0.95)
}

