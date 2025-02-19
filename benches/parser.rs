use criterion::{criterion_group, criterion_main, Criterion};
use oblivion::utils::parser::OblivionRequest;

fn criterion_benchmark_parser_request(c: &mut Criterion) {
    c.bench_function("parser_request", |b| b.iter(|| {
        let header = "GET / Oblivion/2.0";
        OblivionRequest::new(&header).unwrap();
    }));
}

criterion_group!(benches, criterion_benchmark_parser_request);
criterion_main!(benches);
