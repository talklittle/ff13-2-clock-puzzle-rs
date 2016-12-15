use std::env;

fn main() {
    let mut args = env::args();
    args.next();  // skip the command name (arg 0)

    let mut clock_vals:Vec<i32> = vec![];
    for arg in args {
        clock_vals.push(arg.parse().unwrap());
    }
    let count = clock_vals.len() as i32;

    let mut all_paths = vec![];

    let mut i = 0;
    for val in clock_vals {
        let low_destination  = (i - val) % count;
        let high_destination = (i + val) % count;
        let destinations =
            if low_destination == high_destination { vec![low_destination] } else { vec![low_destination, high_destination] };
        all_paths.push(destinations);
        i += 1;
    }

    i = 0;
    for dest in all_paths {
        let dest_string_vec = dest.iter().map(|num| { format!("{}", num ) }).collect::<Vec<String>>().join(", ");
        println!("{} => {}", i, dest_string_vec);
        i += 1;
    }
}
