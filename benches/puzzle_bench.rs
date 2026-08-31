use criterion::{black_box, criterion_group, criterion_main, Criterion};
use puzzle_solver::{Board, solve_astar, is_solvable, parse_input};

/// 可解棋盘用例（按最优步数递增排列）
/// 所有用例均已通过 is_solvable 验证，最优步数经独立 BFS 全状态搜索确认
fn solvable_test_cases() -> Vec<(&'static str, &'static str, usize)> {
    vec![
        // (名称, 棋盘字符串, 预期最优步数)
        ("trivial_0steps", "123456780", 0),
        ("easy_1step",     "123456708", 1),
        ("easy_2steps",    "123456078", 2),
        ("easy_4steps",    "152403786", 4),
        ("medium_8steps",  "052183476", 8),
        ("medium_12steps", "582173046", 12),
        ("hard_16steps",   "872503146", 16),
        ("hard_20steps",   "870542163", 20),
        ("hard_23steps",   "241350768", 23),
        ("max_31steps",    "867254301", 31),
    ]
}

fn benchmark_puzzle_solving(c: &mut Criterion) {
    let mut group = c.benchmark_group("solve_astar");
    group.sample_size(100); // 增加采样次数以提高精度

    for (name, input, expected_steps) in solvable_test_cases() {
        let board = parse_input(input).unwrap();
        assert!(is_solvable(board), "用例 {} 应可解", name);

        // 预验证：确保求解器返回正确步数
        let solution = solve_astar(board).unwrap();
        assert_eq!(solution.len(), expected_steps, "用例 {} 步数不匹配", name);

        group.bench_function(name, |b| {
            b.iter(|| {
                black_box(solve_astar(black_box(board)));
            })
        });
    }
    group.finish();
}

fn benchmark_manhattan_distance(c: &mut Criterion) {
    let mut group = c.benchmark_group("manhattan_distance");
    let goal_positions = Board::GOAL_POSITIONS;

    let test_cases = vec![
        ("solved",     "123456780"),
        ("dist_1",     "123456708"),
        ("dist_6",     "123405678"),
        ("dist_12",    "832145670"),
        ("dist_22_max","574802631"),
    ];

    for (name, input) in test_cases {
        let board = parse_input(input).unwrap();

        group.bench_function(name, |b| {
            b.iter(|| {
                black_box(board.manhattan_distance(&goal_positions));
            })
        });
    }
    group.finish();
}

fn benchmark_solvability_check(c: &mut Criterion) {
    let mut group = c.benchmark_group("is_solvable");

    let test_cases = vec![
        ("solvable_trivial", "123456780"),
        ("solvable_medium",  "123450678"),
        ("solvable_hard",    "504871623"),
        ("unsolvable_1",     "123456870"),
        ("unsolvable_2",     "213456780"),
    ];

    for (name, input) in test_cases {
        let board = parse_input(input).unwrap();

        group.bench_function(name, |b| {
            b.iter(|| {
                black_box(is_solvable(black_box(board)));
            })
        });
    }
    group.finish();
}

fn benchmark_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse_input");

    let test_cases = vec![
        ("valid",       "123456780"),
        ("valid_hard",  "504871623"),
        ("invalid_len", "12345678"),
        ("invalid_char","12345678a"),
        ("invalid_dup", "112345678"),
        ("missing_zero","123456789"),
    ];

    for (name, input) in test_cases {
        group.bench_function(name, |b| {
            b.iter(|| {
                let _ = parse_input(black_box(input));
            })
        });
    }
    group.finish();
}

fn benchmark_stress_random(c: &mut Criterion) {
    let mut group = c.benchmark_group("stress_solve");
    group.sample_size(50);

    // 使用多个不同难度的可解棋盘进行压力测试（最优步数经独立 BFS 确认）
    let stress_cases: Vec<(&str, usize)> = vec![
        ("870542163", 20),
        ("241350768", 23),
        ("867254301", 31),
    ];

    for (input, expected) in stress_cases {
        let board = parse_input(input).unwrap();
        let name = format!("stress_{}steps", expected);

        group.bench_function(&name, |b| {
            b.iter(|| {
                black_box(solve_astar(black_box(board)));
            })
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    benchmark_puzzle_solving,
    benchmark_manhattan_distance,
    benchmark_solvability_check,
    benchmark_parsing,
    benchmark_stress_random,
);
criterion_main!(benches);