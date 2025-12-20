use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};

static JSON: &str = include_str!("../examples/samples/sample.json");

fn bench_json(c: &mut Criterion) {
    c.bench_function("str find", {
        move |b| b.iter(|| str_find::find_str(black_box(JSON)))
    });

    c.bench_function("neure find", {
        move |b| b.iter(|| neure_find::find_str(black_box(JSON)))
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().configure_from_args();
    targets = bench_json
);

criterion_main!(benches);

mod str_find {

    pub fn find_str(haystack: &str) {
        assert!(haystack.find(|v: char| v.is_alphanumeric()).is_some());
    }
}

mod neure_find {

    use neure::prelude::*;

    pub fn find_str(haystack: &str) {
        assert!(
            CharsCtx::new(haystack)
                .find(neu::alphanumeric().once())
                .is_some()
        );
    }
}
