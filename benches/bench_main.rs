use benchmarks::bench_convolution::convolutions;
use criterion::criterion_main;
mod benchmarks;

criterion_main!(convolutions);
