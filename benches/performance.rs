use criterion::{black_box, criterion_group, criterion_main, Criterion};

use adventofcode2024::{day1, read_input};

macro_rules! create_day_benchmark {
    ($name:ident, $part:path) => {
        fn $name(c: &mut Criterion) {
            c.bench_function(&format!("day1-{}", stringify!($part)), |b| {
                b.iter(|| black_box($part(&read_input(1))))
            });
        }
    };
}

create_day_benchmark!(benchmark_day1_part1, day1::part1);
create_day_benchmark!(benchmark_day1_part2, day1::part2);

criterion_group!(benches, benchmark_day1_part1, benchmark_day1_part2);
criterion_main!(benches);
