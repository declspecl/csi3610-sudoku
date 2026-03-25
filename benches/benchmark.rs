use std::{fs, path::Path};
use std::str::FromStr;
use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};

use ssolvent::{board::board::Board, solver::solve};

fn load_puzzles(path: &Path) -> Vec<Board> {
    let contents = fs::read_to_string(path)
        .expect(&format!("Failed to read puzzles file `{}`", path.display()));

    contents
        .lines()
        .map(|line| Board::from_str(line).expect(&format!("Failed to parse puzzle from `{}`", line)))
        .collect()
}

fn bench_category(c: &mut Criterion, category: &str, puzzles: &[Board]) {
    let mut index: usize = 0;
    let puzzle_count = puzzles.len();

    c.bench_function(&format!("solve/{category}"), |b| {
        b.iter(|| {
            let board = &puzzles[index];
            index = (index + 1) % puzzle_count;

            let solution = black_box(solve(board));
            debug_assert!(solution.is_some(), "Expected a solution");
        })
    });
}

fn run_benchmark(c: &mut Criterion) {
    let resources_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/puzzles");

    let easy = load_puzzles(&resources_dir.join("easy.txt"));
    let medium = load_puzzles(&resources_dir.join("medium.txt"));
    let hard = load_puzzles(&resources_dir.join("hard.txt"));
    let diabolical = load_puzzles(&resources_dir.join("diabolical.txt"));
    let clue_17 = load_puzzles(&resources_dir.join("17_clue.txt"));
    let norvig_top95 = load_puzzles(&resources_dir.join("norvig_top95.txt"));
    let norvig_hardest = load_puzzles(&resources_dir.join("norvig_hardest.txt"));

    bench_category(c, "easy", &easy);
    bench_category(c, "medium", &medium);
    bench_category(c, "hard", &hard);
    bench_category(c, "diabolical", &diabolical);
    bench_category(c, "17_clue", &clue_17);
    bench_category(c, "norvig_top95", &norvig_top95);
    bench_category(c, "norvig_hardest", &norvig_hardest);
}

criterion_group!(benches, run_benchmark);
criterion_main!(benches);