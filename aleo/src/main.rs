fn main() {


    let course = "Rust".to_owned();


    let color = "Blue";
    let name = String::from("John Doe");
    let person = Person{
        name:name,
        color:color,
        age: 99,
    };

    let value = "value".to_owned();
    print_str_pointer(&value);
    print_str_pointer(&value);
    print_str_borrowed(value)


}

fn print_str_pointer(data: &str) {
    println!("{}", data);
}

fn print_str_borrowed(data: String){
    println!("{}", data);
}


struct Person <'a>{
    name: String,
    color: &'a str,
    age :i32
}

