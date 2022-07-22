macro_rules! function_name {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        &name[..name.len() - 3]
    }}
}

fn print_name(name: &str) {
    println!("{}",name);
}

fn simple_if_else() {
    print_name(function_name!());
    let number = 1;
    if number < 5 {
        println!("number is less");
    } else {
        println!("number is equal or more");
    }
    let number = 5;
    if number < 5 {
        println!("number is less");
    } else {
        println!("number is equal or more");
    }
    // if number { // won't compile because number is not bool
    // }
}
fn simple_if_else_if() {
    print_name(function_name!());
    let number = 11;
    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4/3/2");
    }
}
fn if_in_assignment() {
    print_name(function_name!());
    let number = 10;
    let number = if number != 11 { 11 } else { number - 1};
    println!("number : {number}", );
    // let number = if number != 11 { 11 } else { "manthan" }; // won't compile
}
fn simple_loop(){
    print_name(function_name!());
    let mut counter = 0;
    let number = loop {
        counter += 1;
        if counter == 10 {
            break counter*2;
        }
    };
    println!("number : {number}");
}
fn multiple_loop(){
    print_name(function_name!());
    let mut count = 0;
    'out_loop: loop {
        println!("count : {count}");
        let mut remain = 3;
        loop {
            println!("remain :{remain}");
            if remain < count {
                break;
            }
            if count == 3 {
                break 'out_loop;
            }
            remain -= 1;
        }
        count += 1;
    }
}
fn main() {
    simple_if_else();
    simple_if_else_if();
    if_in_assignment();
    simple_loop();
    multiple_loop();
}
