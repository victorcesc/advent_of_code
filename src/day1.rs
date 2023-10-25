



pub fn get_max_fast(contents: &String) ->  u32 {
   
    let result  = contents
        .split("\n\n")//split the calories of elves
        .map(|block| //map the blocks of calories
            block
                .split("\n")
                .filter(|item| !item.is_empty())//filter to remove the empty data
                .map(|item| item.parse::<u32>().unwrap())//transform every item in u32
                .sum::<u32>()//sum the values inside blocks
        ).max().unwrap();//get the max value and unwrap the value
        
    result // return the result
}
pub fn three_most_caloric(contents: &String) ->  u32{
    let mut largest_values: Vec<u32> = contents
    .split("\n\n")
    .map(|block| {
        block
            .split("\n")
            .filter(|item| !item.is_empty())
            .map(|item| item.parse::<u32>().unwrap())
            .sum::<u32>()
    })
    .collect(); // colect the results in vector

    largest_values.sort_by(|a, b| b.cmp(a)); // inverse sort the vector
    let sum: u32 = largest_values.iter().take(3).sum(); // get three bigger values and sum
    sum // return the sum

}


