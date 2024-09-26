// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-License-Identifier: EUPL-1.2
//
// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen, eza contributors
// SPDX-FileCopyrightText: 2014 Benjamin Sago
// SPDX-License-Identifier: MIT
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("logger", |b| {
        b.iter(|| {
            eza::logger::configure(black_box(std::env::var_os(eza::options::vars::EZA_DEBUG)))
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
