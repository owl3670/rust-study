fn main() {
    println!("Hello, world!");

    another_function();
    another_param_function(30);
    add(3, 5);
}

fn another_function() {
    println!("Another function.");
}

fn another_param_function(x: i32) {
    println!("The value of x is: {x}");
}

fn add(a: i32, b: i32){
    println!("{a} + {b} = {}", a+b);
}
