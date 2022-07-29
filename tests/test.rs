extern crate libtest_mimic;

use libtest_mimic::{Arguments, Test, Outcome, run_tests};


// Parse command line arguments

// Run all tests and exit the application appropriatly (in this case, the
// test runner is a dummy runner which does nothing and says that all s
// passed).
fn main() {
    let args = Arguments::from_args();

    // Create a list of tests (in this case: three dummy tests)
    let tests = vec![
        Test::test("toph"),
        Test::test("sokka"),
        Test {
            name: "long_computation".into(),
            kind: "".into(),
            is_ignored: false,
            // is_ignored: true,
            is_bench: false,
            data: (),
        },
    ];

    run_tests(&args, tests, |test| {
        if test.name != "long_computation" {
            Outcome::Passed
        } else {
            Outcome::Failed { msg: None }
        }
    }).exit();
}
