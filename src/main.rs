use std::ffi::c_void;

fn main()
{
    memory_demo();
    for x in 1..5
    {
        setProfile(x, "Ken", 23, 'M', 400.00f32, false);
    }
}

fn memory_demo()
{
    // Transfer / Moving References or Values
    let a: String = String::from("A Memory");
    let b: String = a;

    // println!("{a}"); Can't call this because a has no value
    println!("{b}");

    // Borrowing
    let c: &String = &b;
    println!("{c}");

}

fn setProfile(index: i8, name: &str, age: i8, gender: char, balance: f32, is_married: bool)
{
    let index_text: String = index.to_string();
    let id: &[u8] = index_text.as_bytes();
    let _gender: &str = if gender == 'M' { "Male" } else { "Female"};
    println!("[{index}] ----------------------------- ");
    println!("User: {name}");
    println!("Age: {age}");
    println!("Gender: {_gender}");
    println!("Balance: {balance}");
    println!("Is Married: {is_married}");
    let mut x: String = String::from("");

    for byte in id.iter()
    {
        x.push_str(byte.to_string().as_str());
    };

    println!("Name bytes: {x}");
}