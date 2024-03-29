use std::env;

fn main() {
    let mut args = env::args();
    args.next();  // skip the command name (arg 0)

    let mut clock_vals:Vec<i32> = vec![];
    for arg in args {
        clock_vals.push(arg.parse().unwrap());
    }

    let result = solve(clock_vals);

    let result_string_vec = result.iter().map(|num| { format!("{}", num ) }).collect::<Vec<String>>().join(", ");
    println!("Result: {}", result_string_vec);
}

fn solve(clock_vals: Vec<i32>) -> Vec<i32> {
    let count = clock_vals.len() as i32;

    let mut all_paths = vec![];
    let mut all_origins = vec![];

    let mut i = 0;
    for val in clock_vals {
        let low_destination  = (i - val + count) % count;  // +count to ensure positive
        let high_destination = (i + val) % count;
        let mut destinations =
            if low_destination == high_destination { vec![low_destination] } else { vec![low_destination, high_destination] };
        destinations.sort();
        all_paths.push(destinations);
        all_origins.push(i);
        i += 1;
    }

    let result = brute_force(all_paths, all_origins, vec![]).unwrap();

    result
}

fn brute_force(remaining_paths: Vec<Vec<i32>>, allowed_origins: Vec<i32>, path_so_far: Vec<i32>) -> Result<Vec<i32>, String> {
    for i in allowed_origins {
        if !path_so_far.contains(&i) {
            let mut new_path_so_far = path_so_far.clone();
            new_path_so_far.push(i);

            // end condition
            if new_path_so_far.len() == remaining_paths.len() {
                return Ok(new_path_so_far);
            }

            let ref dests = remaining_paths[i as usize];
            let mut new_remaining_paths = remaining_paths.clone();
            new_remaining_paths[i as usize] = vec![];

            match brute_force(new_remaining_paths, dests.clone(), new_path_so_far.clone()) {
                Ok(new_path) => return Ok(new_path),
                Err(_) => continue
            }
        }
    }

    Err("Invalid clock".to_string())
}

#[test]
fn solve_12_hour_example_1() {
    let expected = vec![10, 3, 7, 9, 2, 5, 11, 4, 6, 8, 0, 1];
    assert_eq!(expected.as_slice(), solve(vec![1, 5, 3, 4, 2, 6, 2, 2, 4, 5, 5, 5]).as_slice());
}
