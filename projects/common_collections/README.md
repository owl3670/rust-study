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

## Using an Enum to Store Multiple Types

vector 는 하나의 type 의 값만 저장할 수 있습니다.
하지만 enum 을 사용하면 vector 에 여러 type 의 값들을 저장할 수 있습니다.

```rust
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
```

## Dropping a Vector Drops Its Elements

`struct` 와 같이 vector 는 scope 밖으로 벗어나면 drop 됩니다.

```rust
    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here
```

# Storing UTF-8 Encoded Text with Strings

문자열은 바이트의 모음으로 구현되기 때문에 컬렉션의 맥락에서 설명합니다.  
이 섹션에서는 모든 컬렉션 유형에 있는 `String`에 대한 작업에 대해 설명합니다.

## What Is a String?

먼저 *문자열* 이라는 용어가 무엇을 의미하는지 알아야 합니다.  
Rust 코어에는 `str` 이라는 문자열 슬라이스가 유일한 문자열 유형입니다. 이것의 참조형태인 `&str` 가 많이 보입니다.    
`String` 타입은 핵심 언어 기능에 있는 것이 아닌 Rust 의 표준 라이브러리를 통해 제공됩니다.  
이는 UTF-8 로 인코딩된 문자열 타입입니다. Rust 에서 문자열을 말할때는 `String` 과 `&str` 을 모두 말하는 것입니다.  

## Creating a New String

`Vec<T>` 에서 가능한 많은 연산자가 `String` 에도 적용됩니다.  
`String`은 실제로 bytes vextor의 wrapper 이기 때문입니다.  
`String` 을 생성하는 방법은 다음과 같습니다.

```rust
    let mut s = String::new();
```

위에서 `s` 라는 빈 `string`을 생성합니다.  
이 `string`에 데이터를 넣을 수 있습니다.  

종종 `string`에 초기 데이터가 있을 수 있습니다.  
이를 위해 `string` 리터럴처럼 `Display` 특성을 구현하는 모든 유형에서 사용할 수 있는 `to_string` 메소드를 사용합니다.  

```rust
    let data = "initial contents";
    let s = data.to_string();
    // the method also works on a literal directly:
    let s = "initial contents".to_string();
```

또는 `String::from` 함수를 사용할 수 있습니다.

```rust
    let s = String::from("initial contents");
```

## Updating a String

`String` 은 `Vec<T>` 와 같이 사이즈가 증가 할 수 있고 내용도 바뀔 수 있습니다.  
또한  `+` 연산자, `format!` 매크로 등을 이용해 `String` 값 을 연결할 수 있습니다.  

---

* [목차로](../../README.md)
