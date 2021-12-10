# advent-of-code-2021

Advent of Code 2021 Solutions in Rust

## Solutions

- [Day 1: Sonar Sweep](day1)
- [Day 2: Dive!](day2)
- [Day 5: Hydrothermal Venture](day5)
- [Day 6: Lanternfish](day6)
- [Day 7: The Treachery of Whales](day7)
- [Day 8: Seven Segment Search](day8)
- [Day 9: Smoke Basin](day9)
- [Day 10: Syntax Scoring](day10)

## Usage

### Unit Tests

Unit tests are provided for each day's example to verify the solution returns the correct results. Unit tests can be run
either for the whole workspace or for an individual day.

```shell
# Run Unit Tests for All Days
cargo test

# Run Unit Tests for an Individual Day
cargo test -p day1
```

### Real Input Data

Real input data is provided to each day's challenge using the standard input stream (stdin). The package will attempt to
solve part 1 and part 2 of a day's challenge.

```shell
# Solve Part 1 and Part 2 of an Individual Day's Challenge
cargo run -p day1 < day1/input
```
