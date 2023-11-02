use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Write};

// read the file line by line into the vector
pub fn read_file(filename: String) -> Result<Vec<i32>, io::Error> {
    let mut return_vector: Vec<i32> = Vec::new();

    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let converted_line: i32 = line.trim().parse().unwrap();
        return_vector.push(converted_line);
    }

    return Ok(return_vector);
}

// write the vector line by line into the outfile
pub fn write_file(filename: String, output_vector: &mut Vec<i32>) -> Result<(), io::Error> {
    let file_result = File::create(filename);
    let mut file;
    match file_result {
        Ok(f) => file = f,
        Err(e) => return Err(e),
    }
    for i in output_vector {
        write!(file, "{}\n", i)?;
    }
    return Ok(());
}

pub fn parse_arguments() -> Result<HashMap<String, String>, String> {
    let mut ret_val: HashMap<String, String> = HashMap::new();

    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 5 {
        return Err(format!(
            "Invalid number of arguments. Expected 5, got {}.",
            args.len()
        ));
    }

    let mut skip = false;
    for (index, arg) in args.iter().enumerate() {
        if skip {
            skip = false;
            continue;
        }
        match arg.as_str() {
            "-i" => {
                ret_val.insert(String::from("i"), args[index + 1].clone());
                skip = true;
            }
            "-o" => {
                ret_val.insert(String::from("o"), args[index + 1].clone());
                skip = true;
            }
            "--alg=merge" => {
                ret_val.insert(String::from("alg"), String::from("merge"));
            }
            "--alg=quick" => {
                ret_val.insert(String::from("alg"), String::from("quick"));
            }
            _ => {
                return Err(format!(
                    "Invalid Algorithm for Sorting, please use bucket,merge, or quick"
                ));
            }
        }
    }

    // verify HashMap has an inp, out, and alg before returning
    if ret_val.get("i").is_none() || ret_val.get("o").is_none() || ret_val.get("alg").is_none() {
        return Err(format!(
            "Missing either algorithm, input file, or output file in cmd line arguments"
        ));
    }

    return Ok(ret_val);
}
