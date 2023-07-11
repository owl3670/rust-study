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