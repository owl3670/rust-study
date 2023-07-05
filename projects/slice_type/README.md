# The Slice Type

slice 는 reference 의 일종으로 collection 의 일부를 참조하는 reference 입니다. slice 는 reference 이기에 ownership 이 없습니다.

```rust
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
```

위의 함수는 String 의 reference 를 받아 공백을 찾아서 index 를 반환합니다.  
이 함수는 잘 동작하지만 반환 값을 사용할 때 s 의 상태와 무관한 값을 사용할 수 있다는 단점이 있습니다.  

```rust
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // hello 5 글자이므로 5가 반환됩니다.

    s.clear(); // 문자열을 다 지우는 함수

    // s 는 이제 완전히 비어있지만 word 는 여전히 5 를 가지고 있습니다.
}
```

위의 함수 사용 예시를 보면 `s` 는 `clear()` 함수를 사용해 문자열을 비웠지만 `word` 는 여전히 5 를 가지고 있습니다.  
`word` 는 `s` 의 data 와 동기화되지 않기 때문에 발생하는 문제입니다.

rust 에서는 이러한 문제를 string slice 를 통해 해결할 수 있습니다.