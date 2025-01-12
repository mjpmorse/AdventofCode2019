use std::fs::File;
use std::io::BufRead;
use std::path::PathBuf;
use std::cmp::max;


fn read_number(input: &PathBuf) -> anyhow::Result<Vec<i32>> {
    // open the file at the given path
    let file = File::open(input)?;
    // create a buffered reader to read the file
    let reader = std::io::BufReader::new(file);
    // Create a vector to store the numbers, with an initial capacity of 500
    let mut list1: Vec<i32> = Vec::with_capacity(500);

    // iterate over the lines of the file
    for line in reader.lines() {
        // Read the line, returning an error if it fails
        let line = line?;
        // Parse the line as an i32 and push it to the list if successful
        if let Ok(parsed) = line.parse::<i32>() {
            list1.push(parsed);
        }
    }
    // return the list of numbers
    Ok(list1)
}

fn calculate_fuel(list: &Vec<i32>) -> Vec<i32> {
    // since the elements of list are integers,
    // the / of x by 3 will do integer division, i.e. rounding down
    // we then subtract 2 from the result
    list.iter().map(|x| x / 3 - 2).collect::<Vec<i32>>()
}

pub fn part_a(input: &PathBuf) -> anyhow::Result<i32> {
    // Read the list of numbers from the input file
    let list1 = read_number(input)?;

    // calculate the fuel needed for each module
    let fuel_list = calculate_fuel(&list1);

    Ok(fuel_list.iter().sum())
}

pub fn part_b(input: &PathBuf) -> anyhow::Result<i32> {
    // Read the list of numbers from the input file
    let list1 = read_number(input)?;

    // create a variable to store the total fuel needed
    let mut total_fuel = 0;
    for fuel in list1 {
        // assign the remaining fuel to fuel
        let mut remaining_fuel: i32 = fuel;
        while remaining_fuel > 0 {
            let x = max(remaining_fuel / 3 - 2, 0);
            total_fuel += x;
            remaining_fuel = x;
        }
    }

    Ok(total_fuel)
}