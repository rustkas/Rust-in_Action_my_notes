fn main() {
    let penguin_data = "\
common name,length (cm)
Little penguin,33
Yellow-eyed penguin,65
Fiordland penguin,60
Invalid,data
";
    let show_debug = false;
    let records = penguin_data.lines();

    println!("{:<20} | {:>10}", "Common Name", "Length (cm)");
    println!("{:-<21}+{:->12}", "", "");

    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        let fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();
        if show_debug && cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }
        let name = fields[0];
        if let Ok(length) = fields[1].parse::<u8>() {
            println!("{:<20} | {:>10}", name, length);
    
        }
    }
}
