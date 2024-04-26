use ch2_non_base2::{compare_arrays, get_10, get_16, get_2, get_8};

fn main() {
    let show_table = false;
    if show_table {
        println!(
            "| {:>10} | {:>10} | {:>10} | {:>10} |",
            "base 10", "base 2", "base 8", "base 16"
        );
        println!("| {:->11} | {:->11} | {:->11} | {:->11} |", "", "", "", "");

        for i in 0..=255 {
            println!("| {i:>10} | 0b{i:>10b} | 0o{i:>10o} | 0x{i:>10x} |");
        }
    }

    // let mut vector_10 = vec![255];
    // for i in 0..=255 {
    //     vector_10.push(i);
    //     print!("0x{i:x}, ");
    // }
    println!();

    // Create 4 array with number and compare them
    let array_10 = get_10();

    // binary data
    let array_2 = get_2();

    // 8-data
    let array_8 = get_8();
    // 16-data
    let array_16 = get_16();

    compare_arrays(&array_10, &array_2, &array_8, &array_16);
}
