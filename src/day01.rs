use std::fs::File;
use std::io::BufRead;
use std::path::PathBuf;
use std::iter::successors;


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
    let total_fuel: i32 = list1
        // create an iterator from the list
        .iter()
        // map the calculation function to each element in the iterator,
        // recursively while the condition is still met
        .flat_map(|&fuel| // define fuel as what we will refer to the element as
            // this is the mapping function.
            // flat_map produce a long "array" instead of the nested "array"
            // I would get by using map
             {
                 // successor will generate a sequence starting with fuel
                 successors(Some(fuel), |&remaining| {
                     // with each member of the sequence being calculated using
                     let next = remaining / 3 - 2;
                     // this is the stopping condition
                     (next > 0).then_some(next)
                 }).skip(1) // this will skip adding the firs element to the iterator output list
             })
        .sum(); // we sum the total fuel of all the fuel partial sums

    Ok(total_fuel)
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_part_a() {
        // Arrange
        let input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("input")
            .join("day01.txt");

        // Act
        let result = part_a(&input_path).expect("part_a failed"); // Extract the `i32`

        // Assert
        assert_eq!(result, 3348430);
    }

    #[test]
    fn test_part_b() {
        // Arrange
        let input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("input")
            .join("day01.txt");

        // Act
        let result = part_b(&input_path).expect("part_a failed"); // Extract the `i32`

        // Assert
        assert_eq!(result, 5019767);
    }
}