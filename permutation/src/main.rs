fn solve(n: usize, k: usize, i: usize, result: &mut Vec<i32>, used: &mut Vec<bool>) {
//    println!("n={}, k={}, result={:?} used={:?}", n, k, result, used);
    if i >= k {
        println!("answer={:?}", result);
    } else {
        for j in 0..n as usize {
            if !used[j] {
                used[j] = true;
                result[i] = j as i32;
                solve(n, k, i + 1, result, used);
                used[j] = false;
            }
        }
    }
}

fn permutation(n: usize, k: usize) {
    let mut result = vec![0; k];
    let mut used = vec![false; n];
//    result[1] = 100;
//    used[1] = true;
    println!("n={}, k={}, result={:?} used={:?}", n, k, result, used);
    solve(n, k, 0, &mut result, &mut used);
}

fn main() {
    permutation(3, 3);
}
