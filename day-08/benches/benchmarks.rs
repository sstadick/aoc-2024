use day_08::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    part1::process(divan::black_box(
        include_bytes!("../input1.txt"),
    ))
    .unwrap();
}

#[divan::bench]
fn part2() {
    part2::process(divan::black_box(
        include_bytes!("../input2.txt"),
    ))
    .unwrap();
}
