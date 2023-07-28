# Defining and Instantiating Structs

Struct 는 tuple 과 비슷하지만 각 data 의 이름을 명시할 수 있습니다.  
이를 통해 data 가 어떤 의미를 가지는지 명확하게 할 수 있고, data 에 접근할 때 data 의 순서에 의존하지 않아도 됩니다.

Struct 를 정의하려면 `struct` 키워드를 사용하고, 각 data 의 이름과 타입을 명시하면 됩니다.

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

Struct 를 정의한 후 사용하려면 각 field 의 값을 지정하여 instance 를 생성해야 합니다.

```rust
fn main() {
    let user1 = User {
        username: String::from("testname"),
        email: String::from("testemail"),
        sign_in_count: 1,
        active: true,
    };
}
```

Struct 의 field 는 `.` 과 field 의 이름을 사용하여 접근할 수 있습니다.

```rust
fn main() {
    let user1 = User {
        username: String::from("testname"),
        email: String::from("testemail"),
        sign_in_count: 1,
        active: true,
    };

    println!("user1's email: {}", user1.email);
}
```

instance 를 mutable 로 생성한다면 field 의 값을 변경할 수도 있습니다.

```rust
fn main() {
    let mut user1 = User {
        username: String::from("testname"),
        email: String::from("testemail"),
        sign_in_count: 1,
        active: true,
    };

    user1.email = String::from("testemail2");
}
```

여기서 주의해야 할점은 Struct 의 모든 field 가 mutable 이 된다는 것입니다. Rust 에서는 일부 field 만 mutable 하게 할 수 없습니다.

Struct 를 함수안에서 생성하여 return 할 수도 있습니다.

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
```

## Using the Field Init Shorthand

Struct 의 field 와 struct 생성에 전달되는 parameter 혹은 변수의 이름이 같다면 `field init shorthand` 를 사용할 수 있습니다.

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

## Creating Instances From Other Instances With Struct Update Syntax

새로운 instance 를 생성할 때, struct update syntax 를 사용하여 기존 instance 의 field 값을 사용할 수 있습니다.

```rust
fn main() {
    let user1 = User {
        email: String::from("testemail"),
        username: String::from("testname"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("testemail2"),
        username: String::from("testname2"),
        ..user1
    };
}
```

위에 코드에서 `..` 을 사용하여 user1 의 나머지 field 를 사용하여 user2 를 생성하도록 하였습니다.  
해당 구문은 맨 마지막에 위치해야 하지만 field 의 순서와는 상관 없습니다.  

struct update syntax 는 할당처럼 `=` 를 사용하는데 이는 데이터가 이동하기 때문입니다.  
위에 예제에서 user2 는 email 과 username 을 새로 할당하였는데, 만약 user1 의 email 과 username 을 그대로 사용한다면 user1 은 더이상 사용할 수 없게 됩니다.  
active 와 sign_in_count 는 모두 copy 특성을 구현하는 type 이기에 struct update syntax 를 사용해도 무관합니다.

## Using Tuple Structs Without Named Fields to Create Different Types

Struct 를 정의할 때 field 의 이름을 명시하지 않는 tuple struct 를 사용할 수도 있습니다.

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

위에서 `Color` struct 와 `Point` struct 내부가 모두 `i32` 로 동일하더라도 다른 type 으로 취급됩니다.  

## Unit-Like Structs Without Any Fields

Struct 를 정의할 때 field 를 하나도 명시하지 않는 unit-like struct 를 사용할 수도 있습니다.  
unit-like struct 는 data 를 가지지 않는 어떠한 의미만을 위한 type 을 구현해야 할 때 유용합니다. 

```rust
struct UnitLikeStruct;

fn main() {
    let unit_like_struct = UnitLikeStruct;
}
```

# An Example Program Using Structs

사각형의 면적을 구하는 프로그램을 만들면서 우리가 언제 struct 를 사용해야 하는지 알아보도록 하겠습니다.

```rust
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
```

위의 코드에서 사각형의 면적을 구하는 `area` 함수를 만들었는데, 이 함수는 width 와 height 를 parameter 로 받아서 면적을 구합니다.  
그런데 이 함수만 놓고 봤을때는 parameter가 사각형과 관련이 있는지 명확하지 않습니다.  

## Refactoring with Tuples

```rust
fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```

위의 코드에서 `area` 함수는 tuple 을 parameter 로 받아서 면적을 구합니다.  
한개의 인자만 전달하면 되기에 어떤 면에서는 더 개선되었다고 볼 수 있지만 여전히 parameter 가 사각형과 관련이 있는지 명확하지 않습니다.  
또한 너비와 높이를 구분하기 위해 tuple 의 index 를 사용해야 하는데 이는 가독성을 떨어뜨립니다.

## Refactoring with Structs: Adding More Meaning

우리는 struct 를 사용하여 data 의 이름을 명시할 수 있습니다.

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

위의 코드에서 `Rectangle` struct 를 정의하고, `area` 함수의 parameter 를 `Rectangle` struct 의 reference 로 변경하였습니다.  
`Rectangle` struct 는 `width` 와 `height` 라는 field 를 가지고 있기 때문에 너비와 높이를 명확히 구분할 수 있으며,  
area 함수의 parameter 또한 `Rectangle` struct 의 reference 이기 때문에 `Rectangle` 의 면적을 구하는 함수인 것을 명확히 할 수 있습니다.

## Adding Useful Functionality with Derived Traits

아래 코드와 같이 `Rectangle` struct 를 출력해보면 에러가 발생합니다.

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {}", rect1); // error
}
```
`Rectangle` struct 는 `Display` 을 구현하지 않았기 때문인데요,  
Rust 에서는 struct 는 여러 파라미터가 있기에 출력의 방식이 모호하여 `Display` 구현이 제공되지 않습니다.  
대신 `println!` 안에서 포맷을 변경하여 `Debug` trait 을 사용하도록 해보겠습니다.

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1); // error
}
```

이번에도 에러가 발생하는데, 이는 `Rectangle` struct 이 `Debug` trait 을 구현하지 않았기 때문입니다.  
struct 는 기본적으로 `Debug` trait 을 구현하지 않는데 `Debug` trait 을 구현하게 하려면 `#[derive(Debug)]` annotation 을 사용하면 됩니다.

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
}
```

이제 `Rectangle` struct 를 출력할 수 있습니다.  
`Debug` trait 외에도 다른 trait 들을 사용할 수 있습니다.  
[Appendix C](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html) 에서 다른 trait 들을 확인할 수 있습니다.


### Debug print with pretty print

`Debug` trait 을 사용하면 `println!` 을 사용할 때 `{:?}` 를 사용하여 출력할 수 있습니다.  
그런데 이는 `Debug` trait 을 위한 출력이기 때문에 사용자가 읽기 쉬운 형태가 아닐 수 있습니다.  
우리는 `{:#?}` 를 사용하여 `Debug` trait 을 위한 출력을 좀 더 읽기 쉽게 만들 수 있습니다.

```rust
println!("rect1 is {:#?}", rect1);
```

다른 방법으로는 `dbg!` macro 를 사용하는 것이 있습니다.  
`dbg!` macro는 file 과 line number 까지도 출력해줍니다.

```rust
dbg!(&rect1);
```

# Method Syntax

Method 는 함수와 유사하지만 method 는 struct 의 context 에서만 사용할 수 있습니다.  
그리고 함수와는 달리 첫 번째 parameter 는 항상 `self` 를 사용해야 합니다.  
`self`는 method 가 호출되는 struct의 instance 를 가리킵니다.  


