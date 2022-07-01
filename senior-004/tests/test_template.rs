use cli_test_dir::*;
use std::time::{Duration, Instant};

const BIN: &'static str = "./main";

#[test]
fn testcase_1() {
    let n = 9;
    let a = vec![vec![1, 2, 3]];

    test_function(n, a, Some(true));
}

fn test_function(n: usize, a: Vec<Vec<i64>>, success: Option<bool>) {
    let testdir = TestDir::new(BIN, "");

    // Build input
    let mut input = format!("{}\n", n);
    for row in a {
        for val in row {
            input.push_str(&format!("{} ", val));
        }
    }
    input.push_str("\n");

    // Start program
    let start = Instant::now();
    let output = testdir
        .cmd()
        .output_with_stdin(input)
        .tee_output()
        .expect_success();
    let duration = start.elapsed();

    // Processing outputs
    let outs = output.stdout_str().split(" ").collect::<Vec<&str>>();
    let result: usize = outs[0].replace("\n", "").parse().unwrap();

    if let Some(val) = success {
        assert_eq!(result, if val { 0 } else { 1 });
    }
    assert!(duration <= Duration::new(2, 0));
    assert!(output.stderr_str().is_empty());
}
