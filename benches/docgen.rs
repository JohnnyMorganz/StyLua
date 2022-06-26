use criterion::{black_box, criterion_group, criterion_main, Criterion};
use stylua_lib::{format_code, Config, OutputVerification};

pub fn format_docgen(c: &mut Criterion) {
    c.bench_function("format docgen.lua", |b| {
        b.iter(|| {
            format_code(
                black_box(include_str!("./docgen.lua")),
                black_box(Config::default()),
                black_box(None),
                black_box(OutputVerification::None),
            )
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(40);
    targets = format_docgen
}
criterion_main!(benches);
