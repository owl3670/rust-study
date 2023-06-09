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

<img src="./img/trpl04-05.svg" width="600px" height="300px" alt="Image"></img>

코드를 좀 더 자세히 살펴보겠습니다.

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
// Error 를 발생시키는 코드
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

## Mutable References

위에서 마지막에 보았던 code 를 borrowed 된 값이 수정이 가능하도록 다시 작성할 수 있습니다.

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

우선 `s` 변수를 mutable 하게 선언해야 합니다.  
다음으로 함수에 인자를 넘길때 `&mut` 을 사용하여 mutable reference 를 생성합니다.  
함수에서는 `&mut String` 을 받도록 명시합니다.

Mutable reference 에는 한 가지 큰 제약이 있는데, 그것은 한 개의 mutable reference 만 존재할 수 있다는 것입니다.  
아래와 같은 코드는 에러를 발생시키게 됩니다.

```rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

println!("{}, {}", r1, r2);
```

이러한 제한으로 인해 Rust 는 compile 시 race condition 과 같은 것을 방지 할 수 있습니다.

data race condition 은 다음과 같은 세 가지 상황에서 발생합니다.

* 두 개 이상의 포인터가 동시에 동일한 데이터에 access 함
* 포인터 중 하나 이상이 데이터 write 에 사용됨
* 데이터에 대한 access 를 동기화 하는데 사용되는 메커니즘이 없음

data race 는 알 수 없는 동작을 유발 시키고 분석과 수정을 어렵게 합니다.  
Rust 에러한 data race 를 compile 시에 방지합니다.

중괄호를 사용해서 새로운 scope 를 생성하면 두 개의 mutable reference 를 생성할 수 있습니다.

```rust
let mut s = String::from("hello");

{
    let r1 = &mut s;

}

let r2 = &mut s;
```

rust 에서는 mutable 과 immutable reference 를 동시에 생성하는 것도 불가능합니다.

```rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // BIG PROBLEM

println!("{}, {}, and {}", r1, r2, r3);
```

reference 의 범위는 해당 reference가 마지막으로 사용된 시점까지만 유지됩니다.

```rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
println!("{} and {}", r1, r2);
// r1 and r2 are no longer used after this point

let r3 = &mut s; // no problem
println!("{}", r3);
```

## Dangling References

dangling pointer 란 다른 사람에게 제공되었을 수 있는 유효하지 않은 pointer 를 말합니다. pointer 를 사용하는 언어에서는 dangling pointer 를 실수로 생성할 수 있습니다.  
하지만 Rust 에서는 컴파일러가 dangling pointer 를 생성하지 못하도록 막습니다.

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```

위의 코드는 compile error 를 발생시킵니다. 그 이유를 살펴보겠습니다.

```rust
fn dangle() -> &String {
    let s = String::from("hello"); // s 가 새롭게 생성됨

    &s // s 의 reference 를 반환함
} // s 가 scope 를 벗어나 drop 됨. 그러나 reference 는 반환되기에 dangling reference 가 됨
```

`s` 는 scope에서 벗어나면서 drop 되기에 `&s` 는 dangling reference 가 됩니다.  
Rust 에서는 이러한 상황을 허용하지 않기에 error 를 발생시키게 됩니다.

이러한 dangling reference 를 피하기 위해서는 `String` 을 직접 return 하여 ownership 을 넘겨주어야 합니다.

```rust
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
```

## The Rules of References

* 하나의 mutable reference 또는 여러 개의 immutable reference 를 생성할 수 있습니다.
* reference 는 항상 유효해야 합니다.

