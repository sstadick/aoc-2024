use {{crate_name}}::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    part1::process(divan::black_box(
        "../input1.txt",
    ))
    .unwrap();
}

#[divan::bench]
fn part2() {
    part2::process(divan::black_box(
        "../input2.txt",
    ))
    .unwrap();
}
