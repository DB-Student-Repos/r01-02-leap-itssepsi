pub fn is_leap_year(year: u64) -> bool {
    if year % 100 == 0 && year % 400 == 0 {
        return true;
    }
    if year % 100 == 0 {
        return false;
    }
    if year % 4 == 0 {
        return true;
    }
    return false;
}

fn main() {
    // Test the is_leap_year function
    let year = 2024;
    if is_leap_year(year) {
        println!("{} is a leap year.", year);
    } else {
        println!("{} is not a leap year.", year);
    }
}

    //Hello professor, i'm not done with this code but i'll appreciate any comment 
    //from you. one question. theres an error main does not found!!! 
    // i got that. chat gbt said that error E0601 appears when we have not a main 
    // function. so we have to add a main function to our code. line 14 till the end.
    // and another thing. i dont know if i should do something with the test file or not!