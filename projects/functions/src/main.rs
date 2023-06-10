fn main() {
    println!("Hello, world!");

    another_function();
    another_param_function(30);
    add(3, 5);

    let x = 6; // Statement
    println!("The value of x is: {x}");
    let y = {       // Expression
        let z = 3;
        x + z
    };
    println!("The value of y is: {y}");
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
