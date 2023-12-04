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



---

* [목차로](../../README.md)
