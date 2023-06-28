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

위의 코드에서 `&s1` 은 `s1` 의 reference 를 calculate_length 함수에 전달합니다.
위 코드에서 메모리 상태를 그림으로 확인하면 아래와 같습니다.

![reference](./img/trpl04-05.svg)

위의 코드를 좀 더 자세히 살펴보겠습니다.

```rust
let s1 = String::from("hello");
let len = calculate_length(&s1);
```

`&s1` 구문은 소유권이 없는 reference 를 생성합니다. ownership이 없기에 참조가 사용되지 않더라도 drop 되지 않습니다.

다음은 함수에서 벌어지는 일을 확인해보겠습니다.

```rust
fn calculate_length(s: &String) -> usize {
    s.len()
}
```

위의 함수는 `s` 라는 이름으로 `String` 의 reference 를 받습니다.  
`s` 변수는 함수 scope 에서 나가더라도 ownership 이 없기에 drop 되지 않습니다. 
앞서 말했듯이 reference 를 사용한다면 ownership 을 넘기기 위해 return 을 하지 않아도 됩니다.

reference 를 생성하는 작업을 `borrowing` 이라고 합니다.  
만약 우리가 `borrowing` 한 것을 수정하려면 어떻게 될까요?

```rust
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
```

위의 code 는 에러를 발생시키게 됩니다.  
reference 는 변수와 마찬가지로 immutable 이 기본이기 때문입니다.