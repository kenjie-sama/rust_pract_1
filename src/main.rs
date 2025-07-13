use std::io;
use std::io::Write;

fn main()
{
    let fname: String = prompt("Enter name");
    let age: i32 = to_int32(prompt("Enter age"));
    println!("Name Entered is \"{}\".", fname);
    println!("Name Age is \"{}\".", age);

    // let username: &str = "Kenjie";
    // let password: &str = "admin123";
    // let age: u32 = 23u32;
    // let balance: f32 = 700.00f32;
    //
    // let user1: User = create_user(username, password, age, balance);
    // print_user(&user1);
    // println!("Hello, {}!", user1.username);
    //
    // let word: String = String::from("Kenjie");
    // let first_char: Option<char> = get_first_char(&word);
    // let last_char: Option<char> = get_last_char(&word);
    //
    // println!("First char is {:?}", first_char);
    // println!("First char is {:?}", last_char);
}

fn memory_demo()
{
    // Transfer / Moving References or Values
    let a: String = String::from("A Memory");
    let mut b: String = a;

    // println!("{a}"); Can't call this because a has no value
    println!("{b}");

    // Borrowing
    let mut c = &mut b;
    c.push_str(" updated");;
    println!("{c}");
    println!("{b}");

}

fn prompt(message: &str) -> String
{
    let mut input = String::new();
    print!("{message} >> ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    return input;
}
fn to_int32(string: String) -> i32
{
    return string.trim().parse().expect("Please type a number!");
}

fn create_user(username: &str, password: &str, age: u32, balance: f32) -> User
{
    return User{
        username: String::from(username),
        password: String::from(password),
        age,
        balance,
        is_active: true,
    };
}
fn print_user(user: &User)
{
    println!("Username: {}", user.username);
    println!("Password: {}", user.password);
    println!("Age: {}", user.age);
    println!("Age: {}", user.age);
}

fn append(current: &String, other: &str)
{
    let mut result: &String = &current;
    // result.push_str(other)
}


fn get_first_char(string: &String) -> Option<char>
{
    string.chars().next()
}
fn get_last_char(string: &String) -> Option<char>
{
    string.chars().last()
}

struct User
{
    username: String,
    password: String,
    age: u32,
    balance: f32,
    is_active: bool,
}