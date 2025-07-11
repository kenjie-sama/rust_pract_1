use std::ffi::c_void;

fn main() {
    for x in 1..5
    {
        setProfile(x, "Ken", 23, 'M', 400.00f32, false);
    }
}


fn setProfile(index: i8, name: &str, age: i8, gender: char, balance: f32, is_married: bool)
{
    let _gender: &str = if gender == 'M' { "Male" } else { "Female"};
    println!("[{index}] ----------------------------- ");
    println!("User: {name}");
    println!("Age: {age}");
    println!("Gender: {_gender}");
    println!("Balance: {balance}");
    println!("Is Married: {is_married}");
}