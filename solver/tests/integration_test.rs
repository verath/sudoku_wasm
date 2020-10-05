use solver;

#[test]
fn solve_simple() {
    assert_eq!(
        solver::solve(solver::SIMPLE).unwrap(),
        solver::SIMPLE_SOLUTION
    );
}
