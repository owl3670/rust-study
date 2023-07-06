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

## String Slices

string slice 는 string 의 일부를 참조하는 reference 입니다.  
아래는 string slice 를 사용하는 예시입니다.

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

위의 코드에서 `s` 와 `world` 의 메모리 구조를 그림으로 나타내면 아래와 같습니다.

![string slice](./img/trpl04-06.svg)

Rust 에서는 `..` 를 사용해 범위를 지정할 수 있습니다.  
`start_index..end_index` 의 형태로 범위를 지정할 수 있으며, start_index 는 범위에 포함되지만 end_index 는 범위에 포함되지 않습니다. (start_index <= 범위 < end_index)  
범위를 지정할 때 시작점과 끝점을 생략할 수도 있습니다.  
또한 변수를 통해 범위를 지정할 수도 있습니다.

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let hello = &s[..5];

let world = &s[6..];
let world = &s[6..11];

let len = s.len();
let world = &s[6..len];

let slice = &s[0..len];
let slice = &s[..];
```

