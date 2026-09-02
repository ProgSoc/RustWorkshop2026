mod parts;
mod solutions;

fn main() {
    let use_solution = true;
    let part = 1;

    match part {
        1 => {
            if use_solution {
                solutions::part1::main()
            } else {
                parts::part1::main()
            }
        }
        2 => {
            if use_solution {
                solutions::part2::main()
            } else {
                parts::part2::main()
            }
        }
        3 => {
            if use_solution {
                solutions::part3::main()
            } else {
                parts::part3::main()
            }
        }
        4 => {
            if use_solution {
                solutions::part4::main()
            } else {
                parts::part4::main()
            }
        }
        _ => println!("Please ensure `part` is an integer from 1 to 4."),
    }
}
