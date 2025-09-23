fn permutation_nested(n: usize, k: usize, callback: fn(&Vec<i32>)) {
    println!("n={}, k={}", n, k);
    let mut result = vec![0; k];
    let mut used = vec![false; n];
    println!("result={:?}, used={:?}", result, used);
    fn solve(n: usize, k: usize, i: usize,
            result: &mut Vec<i32>, used: &mut Vec<bool>, callback: fn(&Vec<i32>)) {
        if i >= k {
            callback(&result);
        } else {
            for j in 0..n as usize {
                if !used[j] {
                    used[j] = true;
                    result[i] = j as i32;
                    solve(n, k, i + 1, result, used, callback);
                    used[j] = false;
                }
            }
        }

    }
    solve(n, k, 0, &mut result, &mut used, callback);
}

type PermutationCallback = fn(&Vec<i32>);

fn permutation_callback_solve(n: usize, k: usize, i: usize,
        result: &mut Vec<i32>, used: &mut Vec<bool>, f: PermutationCallback) {
    if i >= k {
        f(result);
    } else {
        for j in 0..n as usize {
            if !used[j] {
                used[j] = true;
                result[i] = j as i32;
                permutation_callback_solve(n, k, i + 1, result, used, f);
                used[j] = false;
            }
        }
    }
}

fn permutation_callback(n: usize, k: usize, f: PermutationCallback) {
    println!("n={}, k={}", n, k);
    let mut result = vec![0; k];
    let mut used = vec![false; n];
    println!("result={:?}, used={:?}", result, used);
    permutation_callback_solve(n, k, 0, &mut result, &mut used, f);
}

fn permutation_return_solve(n: usize, k: usize, i: usize,
        result: &mut Vec<i32>, used: &mut Vec<bool>, answer: &mut Vec<Vec<i32>>) {
    if i >= k {
        answer.push(result.clone());
    } else {
        for j in 0..n as usize {
            if !used[j] {
                used[j] = true;
                result[i] = j as i32;
                permutation_return_solve(n, k, i + 1, result, used, answer);
                used[j] = false;
            }
        }
    }
}

fn permutation_return(n: usize, k: usize) -> Vec<Vec<i32>> {
    println!("n={}, k={}", n, k);
    let mut answer: Vec<Vec<i32>> = Vec::new();
    let mut result = vec![0; k];
    let mut used = vec![false; n];
    permutation_return_solve(n, k, 0, &mut result, &mut used, &mut answer);
    answer
}

fn permutation_print_solve(n: usize, k: usize, i: usize, result: &mut Vec<i32>, used: &mut Vec<bool>) {
//    println!("n={}, k={}, result={:?} used={:?}", n, k, result, used);
    if i >= k {
        println!("answer={:?}", result);
    } else {
        for j in 0..n as usize {
            if !used[j] {
                used[j] = true;
                result[i] = j as i32;
                permutation_print_solve(n, k, i + 1, result, used);
                used[j] = false;
            }
        }
    }
}

fn permutation_print(n: usize, k: usize) {
    let mut result = vec![0; k];
    let mut used = vec![false; n];
//    result[1] = 100;
//    used[1] = true;
    println!("n={}, k={}, result={:?} used={:?}", n, k, result, used);
    permutation_print_solve(n, k, 0, &mut result, &mut used);
}

fn main() {
    println!("*** permutation_print");
    permutation_print(3, 3);
    println!("*** permutation_return");
    let p = permutation_return(3, 3);
    println!("p={:?}", p);
    let callback = |result: &Vec<i32>| 
        println!("callback result={:?}", result);
    println!("*** permutation_callback");
    permutation_callback(3, 3, callback);
    println!("*** permutation_nested");
    permutation_nested(3, 3, callback);
}
