# Variables and Mutability

Rust 에서 변수는 기본적으로 불변입니다.  
만약 불변 변수에 값을 변경하려고 하면 에러가 발생합니다.

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6; // error
    println!("The value of x is: {}", x);
}
```

만약 변수의 값이 변경되어야 한다면 `mut` 키워드를 사용하여 변수를 가변으로 선언해야 합니다.

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6; // ok
    println!("The value of x is: {}", x);
}
```