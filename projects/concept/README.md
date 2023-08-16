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

## Constants

Rust 에서는 `const` 키워드를 사용하여 상수를 선언할 수 있습니다.  
상수는 불변 변수와 유사하지만 몇 가지 다른점이 있습니다.  

우선 상수는 `mut` 키워드를 사용할 수 없습니다. 그리고 상수는 반드시 type 을 명시해야 합니다.  
상수는 어떤 scope 에서도 선언 가능하고, 상수는 반드시 상수 표현식만 가능합니다.  

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

위는 상수의 예시입니다. 상수는 대문자와 언더스코어를 사용하여 이름을 적는 것이 권장 됩니다.  

상수는 선언된 scope 내에서는 프로그램이 동작하는 동안 유효합니다.  

