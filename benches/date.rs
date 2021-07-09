use criterion::{black_box, criterion_group, criterion_main, Criterion};
use stylua_lib::{format_code, Config, OutputVerification};

pub fn format_date(c: &mut Criterion) {
    c.bench_function("format date.lua", |b| {
        b.iter(|| {
            format_code(
                black_box(include_str!("./date.lua")),
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
    targets = format_date
}
criterion_main!(benches);
