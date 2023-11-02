mod parse;
mod sort;

use parse::{parse_arguments, read_file, write_file};
use sort::{merge_sort, quick_sort};

fn main() -> Result<(), String> {
    let mut input_data: Vec<i32>;

    // parse arguments
    let arguments;
    let arguments_results = parse_arguments();
    match arguments_results {
        Ok(args) => arguments = args,
        Err(e) => return Err(e),
    }

    // read a file into a vector/array ("i" key in arguments HashMap)
    let input_filename = arguments.get("i").unwrap().to_string();
    let infile_success = read_file(input_filename);
    match infile_success {
        Ok(data) => input_data = data,
        Err(_) => return Err(format!("Error reading file")),
    }

    // call the designated sorting function
    match arguments.get("alg").unwrap().as_str() {
        "merge" => merge_sort(&mut input_data),
        "quick" => quick_sort(&mut input_data),
        _ => return Err(format!("Invalid sorting algorithm")),
    }

    // write the sorted elements into the output file ("o" key in arguments HashMap)
    let output_filename = arguments.get("o").unwrap().to_string();
    let outfile_success = write_file(output_filename, &mut input_data);
    match outfile_success {
        Ok(_) => println!("Successfully printed file"),
        Err(_) => {
            println!("Error printing file");
            return Err(format!("Invalid output file name or configuration"));
        }
    }
    return Ok(());
}
