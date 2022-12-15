use std::env;
use std::fs;

struct Elf {
    Calories: u32,
}

// impl Elf {
//     pub fn new(Calories: u32) -> Self {
//         Self { Calories }
//     }
// }

fn main() {
    println!("Start of The Journey");
    let input: String = fs::read_to_string("C:/GitHub/Advent-of-Code-2022/Day1/src/Input.txt")
        .expect("Should have been able to read the file");
    // let mut CurrentElf: Elf = Elf { Calories: 2 };

    let mut elfs: Vec<Elf> = Vec::new();

    let mut current_elf: Elf = Elf { Calories: 0 };

    let mut most_calorie_filled_elfs_index: u32 = 0;
    let mut most_calorie_filled_elfs_amount: u32 = 0;
    let mut elf_index: u32 = 0;

    for n in input.lines() {
        if n == "" {
            if (current_elf.Calories > most_calorie_filled_elfs_amount) {
                most_calorie_filled_elfs_amount = current_elf.Calories;
                most_calorie_filled_elfs_index = elf_index;
            }
            elfs.push(current_elf);
            current_elf = Elf { Calories: 0 };
            elf_index += 1;
        } else {
            current_elf.Calories += str::parse::<u32>(n).unwrap();
        }
    }

    println!(
        "most_calorie_filled_elfs_index: {}",
        most_calorie_filled_elfs_index
    );
    println!(
        "most_calorie_filled_elfs_amount: {}",
        most_calorie_filled_elfs_amount
    );

    // Part two
    //     println!("Part two");
    //     let mut top3_elfs: [u32; 3] = [0, 0, 0];
    //     let mut top3_elfs_index: [u32; 3] = [0, 0, 0];
    //     current_elf = Elf { Calories: 0 };
    // elf_index = 0;

    //     for n in input.lines() {
    //         if n == "" {
    //             for elf_3index in 0..3 {
    //                 if (current_elf.Calories > top3_elfs[elf_3index as usize]) {
    //                     println!("Just found {}", current_elf.Calories);
    //                     top3_elfs[elf_3index as usize] = current_elf.Calories;
    //                     top3_elfs_index[elf_3index as usize] = elf_index;
    //                     break;
    //                 }
    //             }
    //             elfs.push(current_elf);
    //             current_elf = Elf { Calories: 0 };
    //             elf_index += 1;
    //         } else {
    //             current_elf.Calories += str::parse::<u32>(n).unwrap();
    //         }
    //     }

    //     println!("{}", (top3_elfs[0] + top3_elfs[1] + top3_elfs[2]));
    //     println!("{}", (top3_elfs[0]));
    //     println!("{}", (top3_elfs[1]));
    //     println!("{}", (top3_elfs[2]));

    // Part 2 attepts 2
    let mut elf_values: Vec<usize> = Vec::new();

    let mut current_calories: usize = 0;

    // acumelate all the difrent elf values
    for n in input.lines() {
        if n == "" {
            elf_values.push(current_calories);
            current_calories = 0;
        } else {
            current_calories += str::parse::<usize>(n).unwrap();
        }
    }

    // finde the 3 heighest values
    let mut top3_elfs: [usize; 3] = [0, 0, 0];

    for elf_val in elf_values {
        for n in 0..3 {
            if elf_val > top3_elfs[n] {
                top3_elfs[n] = elf_val;
                break;
            }
        }
    }

    println!("{}", (top3_elfs[0] + top3_elfs[1] + top3_elfs[2]));
    println!("{}", (top3_elfs[0]));
    println!("{}", (top3_elfs[1]));
    println!("{}", (top3_elfs[2]));

    // println!("{}", (top3_elfs[0] + top3_elfs[1] + top3_elfs[2] + top3_elfs[3]));
    // println!("{}", (top3_elfs[0]));
    // println!("{}", (top3_elfs[1]));
    // println!("{}", (top3_elfs[2]));
    // println!("{}", (top3_elfs[3]));
}
