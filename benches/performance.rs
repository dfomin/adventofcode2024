use criterion::{black_box, criterion_group, criterion_main, Criterion};

use adventofcode2024::read_input;

macro_rules! create_day_benchmark {
    ($day:expr) => {
        paste::paste! {
            fn [<benchmark_day $day _part1>](c: &mut Criterion) {
                c.bench_function(&format!("day{}-part1", $day), |b| {
                    b.iter(|| black_box(adventofcode2024::[<day$day>]::part1(&read_input($day))))
                });
            }

            fn [<benchmark_day $day _part2>](c: &mut Criterion) {
                c.bench_function(&format!("day{}-part2", $day), |b| {
                    b.iter(|| black_box(adventofcode2024::[<day$day>]::part2(&read_input($day))))
                });
            }

            criterion_group!(benches, [<benchmark_day $day _part1>], [<benchmark_day $day _part2>]);
        }
    };
}

create_day_benchmark!(16);

criterion_main!(benches);
