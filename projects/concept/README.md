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

### The Character Type

Rust 는 4 byte 로 문자를 표현하는 `char` 자료형이 있습니다.  
ASCII 보다 많은 문자를 표현 가능하고 한국어, 이모지 까지 가능합니다.  
홀따옴표(`)를 통해 값을 표기해야 합니다.  

```rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';
}
```

## Compound Types

compound type 은 여러 값은 하나의 type 으로 묶을 수 있으며, Rust 에서는 tuple, array 두 가지가 있습니다.  

### The Tuple Type

tuple 은 다양한 type 을 하나의 compound type 으로 grouping 할 수 있는 일반적인 방법입니다.  

```rust
fn main(){
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

tuple 은 type 명시를 생략할 수도 있습니다.  

```rust
fn main() {
    let tup = (500, 6.4, 1);
}
```

tuple 에 있는 값은 destructing 을 통해 개별 값으로 분리하여 사용할 수 있습니다.

```rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}"); // 6.4
}
```

tuple 의 값에 직접 접근하여 사용할 수도 있습니다.  

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

### The Array Type

array 는 tuple 과는 달리 같은 type 의 value 들을 collection 으로 만드는 방법입니다.  

```rust
fn main(){
    let a = [1, 2, 3, 4, 5];
}
```

array 는 데이터를 힙이 아닌 스택에 할당하거나, 항상 고정된 수의 사이즈를 확보하고 싶을 때 유용합니다.  
달력과 같이 고정된 길이의 사이즈의 collection 을 만들고 싶다면 array 를 사용하면 됩니다.  

```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
"August", "September", "October", "November", "December"];
```

array 는 type과 길이를 명시 할 수 있습니다.  

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

같은 값이 반복되게 초기화 하는 방법도 존재합니다.  

```rust
let a = [3; 5];
```

#### Accessing Array Elements

array 의 값에 접근하기 위해서는 요소의 위치에 해당하는 index 번호를 사용해야 합니다.  

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0]; // 1
    let second = a[1];  // 2
}
```

#### Invalid Array Element Access

array 의 끝을 넘어서 요소에 접근하려면 어떻게 될까요?  

```rust
use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)  // 만약 10을 입력한다면
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];  // index out of bounds 에러가 발생합니다. 

    println!("The value of the element at index {index} is: {element}"); // 코드가 실행되지 않게 됩니다.
}
```

위의 코드에서 index 를 입력 받았을 때 array 의 끝 index 보다 높은 번호를 입력 받는다면 array 에서 값에 접근하려 할 때 런타임 에러가 발생합니다.  

# Functions

Rust 에서 함수는 `fn` 키워드를 사용하여 선언합니다.  

```rust
fn main() {
    println!("Hello, world!");
    
    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

위의 코드에서 `main' 도 함수이며, `another_function` 도 함수입니다. 

## Parameters

함수는 parameter 를 가질 수 있습니다.  
parameter 는 함수의 signature 에서 정의되며, 함수의 body 에서 사용할 수 있습니다.  
함수의 parameter 로 전달되는 값은 argument 라고 부릅니다.  

```rust
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
```

함수의 signature 에서 parameter 는 항상 type 과 함께 명시되어야 합니다.  

## Statements and Expressions

Rust 에서 함수의 body 는 statement 와 expression 으로 이루어져 있습니다.  
statement 는 어떠한 값을 반환하지 않는 문장입니다.
expression 은 어떠한 결과 값을 반환하는 문장입니다.  
예시를 확인해 보겠습니다.  

```rust
fn main(){
    let y =6; // statement
}
```

위의 코드에서 `let y = 6` 은 statement 로 반환값이 없는 문장입니다.

```rust
fn main(){
    let y = {
        let x = 3;
        x + 1 
    };

    println!("The value of y is: {y}"); // 4
}
```

expression 은 값으로 평가되는 문장으로 위의 코드에서 `x + 1` 은 expression 입니다.  
심지어 `let x = 3;` 에서 3 자체도 statement 에 속한 expression 이며, 함수를 호출하는 것, macro를 호출하는 것 도 expression 입니다.  
위의 코드에서는 중괄호로 묶은 block 이 보이는데 이 또한 expression 이기에 `y` 변수에 결과값을 담아 사용할 수 있는 것입니다.  
expression 은 마지막에 세미콜론을 붙이지 않습니다.