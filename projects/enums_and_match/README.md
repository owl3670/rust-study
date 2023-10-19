# Defining an Enum

struct 는 연관있는 field 와 data 를 함께 묶을 수 있는 방법을 제공했습니다.  
enum 은 value 가 특정한 value 집합 중 하나라는 것을 말할 수 있는 방법을 제공합니다.  
예를 들어 사과는 귤, 포도, 배와 같은 과일의 집합 중 하나라고 말할 수 있습니다.  

IP Address 는 V4 와 V6 가 있는데 IP 주소 표현 방법은 둘 중 하나이여야 하지 둘 다 일수는 없습니다.  
이러한 성격의 데이터 구조를 표현할 때 enum 이 유용합니다.  

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

## Enum Values

IP Address Kind enum 과 struct 를 함께 사용하여 IP Address 를 표현할 수 있습니다.

```rust
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};
```

위와 같이 enum 과 struct 를 통해 IP Address 를 표현할 수 있으며, IP Address 를 표현하는데에 좋습니다.  
rust 에서는 enum 만을 사용하여 위와 같은 표현을 더 단순화 시키는 방법도 있습니다.  

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
```

위와 같은 구조에서는 enum 의 각 variant 에 data 를 직접 저장할 수 있습니다.  
이와 같은 동작이 가능한 이유는 enum 의 variant 이름이 enum 의 인스턴스를 생성하는 함수가 되기 때문입니다.  
즉, `IpAddr::V4()` 는 `String` 타입의 인자를 받아 `IpAddr` 타입의 인스턴스를 생성합니다.  

struct 대신 enum 을 사용할 때 또 다른 이점도 있습니다.  
각 variant 마다 관련 데이터의 type 과 수가 다를 수 있다는 점입니다.  
예를 들어 'V4' variant 는 0 ~ 255 사이의 값을 가지는 4개의 숫자로 구성된 IP Address 를 가지며,  
'V6' variant 는 하나의 string 값으로 표현하고 싶을 때 이는 struct 로 표현할 수 없습니다.  
하지만 enum 을 사용하면 다음과 같이 표현할 수 있습니다.

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));
```

위의 코드와 같이 enum 을 사용하면 각 variant 마다 다른 type과 수의 데이터를 가질 수 있습니다.  
enum 에는 어떤 type 의 data 라도 variant 에 사용할 수 있습니다.  
다르 예시로 확인 해보겠습니다.  

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

위의 예시에서는 각 variant 가 다른 type 의 data 를 가지고 있습니다.  

* `Quit` variant 는 data 를 가지지 않습니다.
* `Move` variant 는 anonymous struct 를 가지고 있습니다.
* `Write` variant 는 `String` 을 가지고 있습니다.
* `ChangeColor` variant 는 3개의 `i32` 값을 가지고 있습니다.

enum 과 struct 의 또 다른 유사점은 `impl` 블록을 사용하여 method 를 정의할 수 있다는 것입니다.

```rust
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

## The `Option` Enum and Its Advantages Over Null Values

Rust 에는 `Option` 이라는 enum 이 표준 library 에 정의되어 있습니다.  
`Option` type 은 어떤 값일 수도 있고 아무것도 아닐 수도 있는 값을 표현할 때 유용합니다.  

예를 들어 비어 있지 않은 list 의 첫 번째 항목을 요청하면 값을 얻게 됩니다. 반대로 비어있는 list의 첫 번째 항목을 요청하면 아무것도 얻지 못합니다.  
이러한 개념을 type system 으로 표현하면 컴파일러가 처리해야 할 모든 경우를 처리했는지 확인할 수 있습니다.  
Rust 에서는 다른 언어에 있는 null의 개념이 없기에 `Option` enum 을 사용하여 null 의 개념을 표현합니다.  

```rust
enum Option<T> {
    Some(T),
    None,
}
```

`<T>` 구문은 generic type의 parameter 로 나중에 더 자세히 다루겠습니다.  
`Option<T>` enum 은 `Some` variant 와 `None` variant 를 가지고 있습니다.  
`Some` variant 는 `T` type 의 data 를 가지고 있으며, `None` variant 는 data 를 가지고 있지 않습니다.  

```rust
let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;
```

`Option` type은 위와 같이 사용할 수 있습니다.  
그런데 이러한 방식이 null 을 사용하는 것보다 안전할까요?  

```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;
```

위의 코드는 `i8` 타입과 `Option<i8>` 타입을 더하려고 하기 때문에 컴파일 에러가 발생합니다.  
`Option<T>` 타입은 값이 있거나 없는 것을 의미히며, 이때 값이 있다면 명시적으로 type 을 변경하여 사용해야 합니다.  
즉 `Option<T>` 타입이 명시되어 있을때만 값이 있는지 없는지 여부를 신경쓰면 되며, 해당 타입의 값을 다룰 때 `None` 인 경우에 대해 명시적으로 처리해야 합니다.  

# The `match` Control Flow Construct

Rust 에는 `match` 라는 매우 강력한 흐름 제어 구조가 있어 일련의 패턴과 값을 비교한 다음 일치하는 패턴에 따라 코드를 실행 할 수 있습니다.  
패턴은 리터럴 값, 변수 이름, 와일드카드 등으로 구성될 수 있습니다.  

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

위의 코드에서 `match` 키워드를 사용하여 `coin` 이라는 변수의 값을 패턴과 비교합니다.  
이는 `if` 를 사용하는 것과 유사해 보이지만 `if`는 boolean 값에 대한 평가만 할 수 있으나 `match`는 패턴을 사용하여 다양한 경우를 처리할 수 있습니다.  

arm 표현식 중 하나가 실행되면 `match` 표현식은 종료되며, `match` 표현식의 결과는 해당 arm 의 결과가 됩니다.  
arm 표현식이 짧은 경우에는 한 줄에 표현할 수 있지만 긴 경우에는 여러 줄에 걸쳐 표현할 수 있습니다.  

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

## Patterns That Bind to Values

`match` 의 arm 에는 다른 유용한 기능도 있습니다.  
바로 패턴과 일치하는 값의 일부에 바인딩하여 `enum`에서 값을 추출하는 기능입니다.  

2008년까지 미국에서는 quater 가 주별로 모양이 달랐다는데요 이를 `enum` 으로 표현 해보겠습니다.  

```rust
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
```

`Coin::Quarter` 에 `state` 라는 변수를 추가하여 `Coin::Quater` 가 일치하면 `state` 변수가 상태 값에 바인딩되게 할 수 있습니다.  

```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
```

위의 코드에서와 같이 `state` 에 값이 바인딩되면 arm 표현식에서 해당 값을 사용할 수 있게됩니다.  

## Matching with `Option<T>`

`Option<i32>` 를 받아 값이 있으면 그 값에 1을 더하고 없으면 `None` 을 반환하는 함수를 만들어 보겠습니다.  
`match` 함수 덕분에 매우 간단하게 구현 할 수 있습니다.  

```rust
fn plus_one(x: Option<i32>){
    match  { 
        None => None,
        Some(i) => Some(i + 1),
    }
}
```

`match` 와 `enum` 을 함께 쓰면서 `enum` 내부 데이터에 변수를 바인딩하여 사용하는 것은 Rust 에서는 매우 자주 있는 일입니다.  

## Matches Are Exhaustive

`match`의 arm의 패턴은 가능성있는 모든 것을 커버해야 합니다.  

```rust
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
        }
    }
```

위의 예제에서는 `None`의 경우에 대해 다루지 않습니다.  
이러한 코드는 버그를 발생시키기 쉬운데 다행히 Rust 에서는 컴파일 에러로 위험을 알려줍니다.  

---

* [목차로](../../README.md)
