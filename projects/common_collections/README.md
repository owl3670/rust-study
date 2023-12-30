# Storing Lists of Values with Vectors

첫 번째로 살펴볼 컬렉션 유형은 벡터라고도 하는 `Vec<T>` 입니다.  
벡터를 사용하면 메모리에서 모든 값을 나란히 배치하는 단일 데이터 구조에 둘 이상의 값을 저장할 수 있습니다.  
벡터는 같은 유형의 값만 저장할 수 있습니다.

## Creating a New Vector

빈 벡터를 생성하기 위헤 `Vec::new` 함수를 호출할 수 있습니다.

```rust
    let v: Vec<i32> = Vec::new();
```

벡터를 생성할 때 값을 들게 하고 싶다면 다음과 같이 할 수 있습니다.

```rust
    let v = vec![1, 2, 3];
```

## Updating a Vector

벡터에 값을 추가하고 싶으면 `push` 메소드를 사용할 수 있습니다.  

```rust
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
```

## Reading Elements of Vectors

vector 에 저장된 값을 참조하기 위해서는 두 가지 방법이 있습니다.  
하나는 `get` 메소드를 사용하는 것이고 다른 하나는 `&` 와 `[]` 를 사용하는 것입니다.  

```rust
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
```

Rust 에서 `[]` 를 통해서 없는 요소에 접근할 때는 panic 이 발생하지만 `get` 메소드를 사용할 때는 panic 이 발생하지 않습니다.  
`get` 메소드를 사용할 때는 `Option<&T>` 이 리턴 됩니다.  

vector 에서 요소에 대한 immutable 참조가 있을 때 vector 에 요소를 추가하려고 하면 컴파일 에러가 발생합니다.  

```rust
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    v.push(6); // Error!

    println!("The first element is: {}", first);
```

## Iterating over the Values in a Vector

vector 의 요소를 순회하기 위해서는 `for` 를 사용할 수 있습니다.  

```rust
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
```

vector 의 요소를 mutable 하게 순회하기 위해서는 `for` 와 `mut` 을 사용할 수 있습니다.  

```rust
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
```

---

* [목차로](../../README.md)
