
# Ownership Rules

* Rust 에서 각 변수는 owner 를 가지고 있음
* owner 는 한 번에 하나만 있을 수 있음
* owner 가 scope 를 벗어 났을때, 변수는 삭제됨

# Variable Scope

변수는 선언된 시점부터 현재 범위가 끝날 때까지 유효합니다.

```rust
{                  
    let s = "hello";
} // s 는 scope 밖으로 벗어나서 삭제됨
```

