fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    let s = String::from("hello"); // s 가 scope 에 들어옴

    takes_ownership(s); // s 의 값이 함수로 이동함. 따라서 이 시점에서 s 는 더이상 유효하지 않음.
    // println!("{}", s); // error!

    let x = 5; // x 가 scope 에 들어옴

    makes_copy(x); // x 는 함수로 이동되지만, i32 는 Copy 이므로 이후에도 x 를 계속 사용할 수 있음.
    println!("{}", x); // ok!
}

fn takes_ownership(some_string: String) { // some_string 변수가 scope 에 들어옴
    println!("{}", some_string);
} // 여기서 some_string 이 scope 밖으로 벗어나고 `drop` 이 호출됨. some_string 의 backing memory 가 해제됨.

fn makes_copy(some_integer: i32) { // some_integer 변수가 scope 에 들어옴
    println!("{}", some_integer);
} // 여기서 some_integer 가 scope 밖으로 벗어나지만, 별다른 일이 발생하지 않음.
