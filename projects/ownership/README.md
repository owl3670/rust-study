# What is Ownership?

Ownership 은 Rust 가 메모리를 관리하는 방법입니다.  
Memory 관리는 보통 garbage collector 를 두던가 명시적으로 할당과 해제를 기재하는 방식을 사용하던가 해서 관리하게 됩니다.  
Rust 는 다른 언어들과는 다르게 메모리를 관리합니다. rust 에서는 ownership 이라는 규칙을 두어 compiler 가 확인하게 합니다.  
이 규칙이 지켜지지 않는다면 compile 되지 않습니다.


## Ownership Rules

* Rust 에서 각 변수는 owner 를 가지고 있음
* owner 는 한 번에 하나만 있을 수 있음
* owner 가 scope 를 벗어 났을때, 변수는 삭제됨

## Variable Scope

변수는 선언된 시점부터 현재 범위가 끝날 때까지 유효합니다.

```rust
{
let s = "hello";
} // s 는 scope 밖으로 벗어나서 삭제됨
```

# Memory and Allocation

변수가 범위를 벗어나면 Rust 에서 drop 이라는 특별한 함수를 호출하게 됩니다.  
이러한 패턴은 Rust 코드 작성 방식에 큰 영향을 미치고 힙에 할당된 데이터를 여러 변수가 사용하게 하려는 복잡한 상황에서 예상치 못한 결과를 초래할 수도 있습니다.

## Variables and Data Interacting with Move

```rust
let x = 5;
let y = x;
```
우리는 위와 같은 상황에서 x 에 5 라는 값이 할당되고 x 에 있던 값이 복사되어 y 에 할당된다는 것을 추측할 수 있습니다.  
정수는 고정된 크기를 가지고 있는 단순한 값이고, 이 두 개의 5 라는 값은 스택에 푸시되고 있습니다.

이제는 `String` 의 경우를 보겠습니다.

```rust
let s1 = String::from("hello");
let s2 = s1;
```

위의 코드는 어떻게 동작할까요?  
int 형 변수를 할당 하고 복사하는 것과 같은 방식으로 동작할 것 같지만 그렇지 않습니다.

<img src="./img/trpl04-01.svg" width="450px" height="350px" alt="Image"></img>

위 그림은 s1 의 메모리 구조를 그림으로 나타낸 것입니다.  
그림에서 왼쪽은 stack 영역 오른쪽은 heap 영역을 의미합니다.  
String 변수의 stack 메모리 구조를 살펴보면 위 그림 왼쪽과 같이 세 가지 부분으로 나뉩니다.  
ptr 은 실제 값이 저장된 메모리의 주소를 가리키고, len 은 contents 가 사용하고 있는 메모리의 길이를 나타내고, capacity 는 allocator 로 부터 할당된 메모리의 크기를 나타냅니다.

이제 s1 을 s2 에 할당하면 어떤 일이 일어나는지 살펴보겠습니다.

<img src="./img/trpl04-02.svg" width="450px" height="600px" alt="Image"></img>

위 그림에서 볼 수 있듯이 s1 의 값이 s2 로 복사되는 것이 아니라 s1 의 ptr, len, capacity 가 s2 로 복사됩니다.  
즉 heap 의 data 는 복사되지 않고 heap 주소를 가르키는 ptr 이 복사되는 것입니다.

<img src="./img/trpl04-03.svg" width="450px" height="600px" alt="Image"></img>

위 그림과 같이 heap 의 data 도 복사하면 안되는 것일까요?  
만약 Rust 가 위와 같이 동작한다면 `s2 = s1` 과 같은 연산을 수행할 때 heap 의 data 가 크다면 많은 시간이 소요될 것입니다.  
때문에 Rust 에서는 위와 같이 동작하지 않습니다.

앞서 설명하기를 Rust 에서는 변수가 범위를 벗어났을 때 drop 이라는 특별한 함수를 호출한다고 했습니다.  
그런데 두 개의 변수가 같은 메모리를 가르키고 있다면 두 개의 변수가 범위를 벗어나면 두 번의 drop 을 호출하게 됩니다.  
이는 이중 해제 오류를 발생시키고 앞서 언급한 메모리 안전 버그 중 하나입니다.

<img src="./img/trpl04-04.svg" width="450px" height="600px" alt="Image"></img>

Rust 에서는 이러한 문제를 해결하기 위해 `s2 = s1` 과 같은 연산을 수행할 때 위 그림과 같이 s1 의 유효성을 무효화 시킵니다.  
때문에 s1 이 범위를 벗어나도 drop 을 호출하지 않습니다.  

다른 언어에서 *shallow copy* 와 *deep copy* 에 대해 공부해보신적이 있으실 수도 있습니다.  
Rust 에서는 복사 후 첫 번째 변수를 무효화 시키기에 *move* 라는 표현을 사용합니다.

## Variables and Data Interacting with Clone

만약 String 의 heap data 를 deep copy 하고 싶다면 `clone` method 를 사용하면 됩니다.

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

## Stack-Only Data: Copy

```rust
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);
```

위의 코드에서는 clone 의 호출이 없었음에도 x 가 유효한 것을 알 수 있습니다.  
이는 int 와 같은 고정된 크기를 가지는 타입은 stack 에 저장되기 때문입니다.  
때문에 실제 값이 빠르게 복사가 되며 x 가 무효하도록 할 이유가 없습니다.

Rust 에는 Integer 와 같이 stack 에 저장된 type 에 `Copy` trait 이라는 특수 annotation 이 있고, 
이러한 특성을 구현한 Type 은 복사되어 다른 변수에 할당된 후에도 유효할 수 있습니다.

일반적으로 단순한 scalar 값들은 `Copy` trait 을 구현할 수 있으며, 할당이 필요하거나 resource의 형태인 것은 `Copy` trait 을 구현할 수 없습니다.
다음은 Copy trait 을 구현하는 type 들의 몇 가지 유형입니다.

- integer types
- boolean types
- floating point types
- character types
- tuples (`Copy` 를 구현하는 유형만 포함된 경우)

# Ownership and Functions

function 으로 변수를 넘기는 동작 방식은 변수를 다른 변수로 할당하는 동작 방식과 유사합니다.  
function 으로 넘겨지는 변수는 `move` 가 되거나 `copy` 가 됩니다.

```rust
fn main() {
    let s = String::from("hello");  // s 변수가 scope 에 들어옴

    takes_ownership(s);             // s 값이 함수로 이동됨
                                    // ... s 는 더이상 유효하지 않음

    let x = 5;                      // x 변수가 scope 에 들어옴

    makes_copy(x);                  // x 값이 함수로 이동됨
                                    // ... x 는 i32 타입이 Copy trait 을 구현하므로 여전히 유효함

} // 여기서 x 가 scope 밖으로 나가고, s 도 scope 밖으로 나감. 하지만 s 는 이미 이동되었으므로, 별다른 일이 발생하지 않음

fn takes_ownership(some_string: String) { // some_string 변수가 scope 에 들어옴
    println!("{}", some_string);
} // 여기서 some_string 이 scope 밖으로 벗어나고 `drop` 이 호출됨. some_string 의 backing memory 가 해제됨.

fn makes_copy(some_integer: i32) { // some_integer 변수가 scope 에 들어옴
    println!("{}", some_integer);
} // 여기서 some_integer 가 scope 밖으로 벗어나지만, 별다른 일이 발생하지 않음.
```

위의 코드에서 `s` 변수를 `takes_ownership` 함수 호출 다음에 사용하려 하면 컴파일 에러가 발생합니다.

# Return Values ans Scope

반환되는 값도 ownership 이 이동됩니다.

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership 함수가 반환하는 값이 s1 으로 이동됨

    let s2 = String::from("hello");     // s2 변수가 scope 에 들어옴

    let s3 = takes_and_gives_back(s2);  // s2 값이 함수로 이동되고, 반환되는 값이 s3 으로 이동됨
} // 여기서 s3 가 scope 밖으로 벗어나고 drop 이 호출됨. s2 가 이동되었으므로 아무 일도 발생하지 않음. s1 이 scope 밖으로 벗어나고 drop 이 호출됨.

fn gives_ownership() -> String {             // gives_ownership 함수가 반환하는 값이 호출한 함수로 이동됨
    let some_string = String::from("hello"); // some_string 변수가 scope 에 들어옴

    some_string                              // some_string 이 반환되고, 호출한 함수로 이동됨
}

// takes_and_gives_back 함수는 String 을 하나 받아서 다른 하나를 반환함
fn takes_and_gives_back(a_string: String) -> String { // a_string 변수가 scope 에 들어옴
    a_string  // a_string 이 반환되고, 호출한 함수로 이동됨
}
```

변수의 ownership 은 항상 같은 패턴을 따릅니다: 다른 변수에 값을 할당하면 변수가 이동합니다.
heap 데이터를 포함하고 있는 변수가 범위를 벗어나면 data 의 소유값이 다른 변수로 이동하지 않는 한 해당 값은 drop 됩니다.

만약 함수가 값을 사용하더라도 ownership 을 유지하고 싶다면, 해당 값을 함수로부터 반환하면 됩니다.
이는 꽤 복잡한 과정이 필요하나 rust 에서는 tuple 을 사용하여 해결할 수 있습니다.

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 메소드는 String 의 길이를 반환함

    (s, length)
}
```

하지만 이는 매우 번거로운 과정인데 다행히도 Rust 에서는 소유권을 이전하지 않고 값을 사용할 수 있는 기능인 reference 를 제공합니다.