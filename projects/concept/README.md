# Variables and Mutability

Rust 에서 변수는 기본적으로 불변입니다.  
만약 불변 변수에 값을 변경하려고 하면 에러가 발생합니다.

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6; // error
    println!("The value of x is: {}", x);
}
```

만약 변수의 값이 변경되어야 한다면 `mut` 키워드를 사용하여 변수를 가변으로 선언해야 합니다.

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6; // ok
    println!("The value of x is: {}", x);
}
```

## Constants

Rust 에서는 `const` 키워드를 사용하여 상수를 선언할 수 있습니다.  
상수는 불변 변수와 유사하지만 몇 가지 다른점이 있습니다.  

우선 상수는 `mut` 키워드를 사용할 수 없습니다. 그리고 상수는 반드시 type 을 명시해야 합니다.  
상수는 어떤 scope 에서도 선언 가능하고, 상수는 반드시 상수 표현식만 가능합니다.  

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

위는 상수의 예시입니다. 상수는 대문자와 언더스코어를 사용하여 이름을 적는 것이 권장 됩니다.  

상수는 선언된 scope 내에서는 프로그램이 동작하는 동안 유효합니다.  

## Shadowing

rust 에서는 이전에 선언한 변수와 같은 이름으로 새 변수를 선언할 수 있습니다.  
첫 번째 변수는 두 번째 변수에 의해 가려집니다.  

```rust
fn main() {
    let x = 5;
    
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}"); // 12
    }
    
    println!("The value of x is: {x}"); // 6
}
```

이러한 Shadowing 은 `mut` 표시와는 다릅니다.  
`let` 키워드를 사용하지 않고 이 변수에 재할당을 시도하면 컴파일 에러가 발생하게 됩니다.  

# Data Types

Rust 에서 모든 값은 어떠한 data type 입니다.  
Rust 는 정적 type 언어이고, 이 말은 변수의 type 이 컴파일 시간에 알 수 있어야 한다는 것입니다.
Rust 에서 type 은 scalar 와 conpound 로 분류할 수 있습니다.

## Scalar Types

scalar type 은 단일값을 나타냅니다. Rust 에서는 integer, floating-point, booleans, character 네 가지 기본 type 이 있습니다.  

### Integer Types

integer 는 signed, unsigned 를 표현할 수 있고 bit 수를 표기해서 길이를 나타낼 수 있습니다.  

| Length | Signed | Unsigned |
|--------|--------|----------|
| 8bit   | i8     | u8       |
| 16bit  | i16    | u16      |
| 32bit  | i32    | u32      |
| 64bit  | i64    | u64      |
| 128bit | i128   | u128     |
| arch   | isize  | usize    |

isize 와 usize 의 길이는 프로그램이 가동되는 컴퓨터의 설계에 따릅니다.  

rust 에서는 integer literal 을 사용하여 자릿수를 쉽게 구분하거나 다른 진법으로 표현 가능합니다.

|Number literals| Example |
|---|---------|
|Decimal| `98_222`  |
|Hex| `0xff`    |
|Octal| `0o77`    |
|Binary| `0b1111_0000` |
|Byte (`u8` only)| `b'A'`   |

rust 에서 integer 기본값은 `i32` 로 설정되어 있습니다.

### Floating-Point Types

Rust 는 floating-point 표현을 위한 두 가지 type 이 있습니다.  
`f32` 와 `f64` 로 각각 32bit, 64bit 의 크기입니다.  
floating-point type 의 default type 은 `f64` 이고 모든 floating-type 은 signed 입니다.  

```rust
fn main() {
    let x = 2.0; // f64
    
    let y: f32 = 3.0; // f32
}
```

### Numeric Operations

Rust 는 기본적인 산술 연산을 지원합니다.  

```rust
fn main() {
    let sum = 5 + 10;
    
    let diff = 95.5 - 4.3;
    
    let product = 4 * 30;
    
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // -1
    
    let remainder = 43 % 5;
}
```

### The Boolean Type

Rust 에서도 `true`, `false` 를 표현할 수 있는 Boolean Type 을 제공합니다.

```rust
fn main() {
    let t = true;
    
    let f: bool = false;
}
```

