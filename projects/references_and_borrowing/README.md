# References and Borrowing

`string` 과 같은 자료형의 변수를 함수에 인자로 넘긴 후에도 사용하고 싶다면 다시 return 을 받는 방식으로 해결할 수 있었습니다.  
rust 에는 이보다 간단한 방법이 있는데 바로 reference 를 사용하는 것입니다.  
reference 는 pointer 와 같은 것으로 변수의 주소값을 가리키는 것이고 개발자는 data 가 저장된 주소값을 따라갈 수 있습니다.  
실제 pointer 와는 다른점도 있는데 reference 는 수명이 유효한 동안 특정 type 의 유효한 값의 주소값을 가르키는 것을 보장한다는 것입니다. 때문에 포인터와 달리 안전하게 사용이 가능합니다.  

reference 사용은 `&` 기호를 사용합니다.  
```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

