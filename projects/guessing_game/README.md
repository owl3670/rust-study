# Setting Up a New Project

실습 프로젝트를 진행하면서 Rust에 대해 더 자세히 알아보겠습니다.  

새로운 프로즈게를 만들기 위해 _projects_ 디렉토리에서 _guessing_game_ 이라는 project 를 만들어 보겠습니다.  

```bash
$ cargo new guessing_game
$ cd guessing_game
```

위의 명령어를 실행하면 _guessing_game_ 디렉토리가 생성되고 _Cargo.toml_ 파일과 _src_ 디렉토리가 생성됩니다.  
_src_ 디렉토리에는 _main.rs_ 파일이 생성되어 있습니다.

# Processing a Guess

guessing game 프로그램의 첫 번째 부분은 user input 을 물어보고 올바른 input 인지 확인하는 것입니다.  

```rust
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
```

user input 을 받은 다음 print 하려면 `io` 라이브러리를 가져와야 합니다.  
`io` 라이브러리는 표준 라이브러리인 `std` 에서 가져 올 수 있습니다.  

```rust
use std::io;
```

Rust 에서는 기본적으로 모든 프로그램에서 가져올 수 있는 표준 라이브러리 집합이 있습니다.  
이 집합을 _prelude_ 라 하며, [표준 라이브러리 문서](https://doc.rust-lang.org/std/prelude/index.html)에서 확인 가능합니다.  

Chapter 1 에서 본 것 처럼 program 의 시작점은 `main` 함수 입니다.  

```rust
fn main(){
    
}
```

`fn` 은 새로운 함수를 정의하는 구문이며, `()` 괄호는 parameter 가 없다는 것을 의미 합니다.  
`{ }` 는 함수의 body 시작과 끝을 나타냅니다.  

`println!` 은 화면에 문자열을 출력하는 매크로입니다.

```rust
    println!("Guess the number!");

    println!("Please input your guess.");
```

## Storing Values with Variables

user input 을 저장하기 위해 변수를 생성했습니다.  

```rust
    let mut guess = String::new();
```

`let` 구문은 변수를 생성하기 위해 사용합니다.  
다른 사용 방법에 대해서도 확인해 보겠습니다.  

```rust
let apples = 5;
```

`apples` 라는 이름의 변수를 생성하면서 5의 값을 할당하고 있습니다.  
Rust 는 기본적으로 변수가 불변입니다. 이는 한번 값을 할당하면 변경이 불가능하다는 뜻입니다.  
만약 변경 가능한 변수를 만들고 싶다면 `mut` 키워드를 붙여주면 됩니다.  

```rust
let apples = 5; // immutable
let mut bannas = 5; // mutable
```

guessing game 으로 돌아와서 `let mut guess` 는 `guess`라는 이름의 변경가능한 변수를 만든다는 구문인 것을 알 수 있습니다.  
euqal sign(`=`) 는 Rust 에게 변수에 어떤값을 할당하길 원한다는 것을 알려주는 것입니다.  
그 오른쪽에 `String::new` 함수는 `String` 의 새로운 객체를 반환하는 함수입니다.  

`::new` 에서 `::` 는 `new`가 String type 연관 함수임을 나타냅니다.  
연관 함수는 특정 type 에서 구현되는 함수입니다.  
많은 type 에서 해당 type 의 새로운 값을 만들어내는 함수로 `new` 를 사용합니다.  

`let mut guess = String::new()` 코드를 전체적으로 다시 분석한다면 변경가능한 `guess` 변수에 새로운 `String` 객체를 할당한다는 의미입니다.

## Receiving User Input

`io` 모듈안에있는 `stdin` 함수를 통해 user 의 input 을 받을 수 있습니다.  

```rust
    io::stdin()
        .read_line(&mut guess)
```

`stdin` 함수는 `std::io::Stdin` 객체를 반환 합니다.  
반환된 객체안의 `read_line` 함수를 통해 user 의 input을 받아 `guess` 변수에 할당합니다.  
`&` 지시자는 _reference_ 를 의미합니다. _refernece_ 는 memory 의 복사 없이 data 에 접근 하는 방법입니다.  
_reference_ 또한 default 가 불변이기에 `&mut` 로 변경 가능한 _reference_ 를 표시하여 함수에 전달합니다.  

## Handling Potential Failure with the `Result`

다음의 코드를 한번 살펴보겠습니다.  

```rust
io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
```

`read_line` 함수는 user 의 입력을 문자열로 전달도 하지만 `io::Result` 를 반환하기도 합니다.  
`Result` 는 `enum` type 입니다.  
`enum` 은 미리 정해둔 여러개의 값 후보를 가질 수 있는 type입니다.  

`Result` 는 `Ok` 와 `Err` 라는 두가지 variant 를 가집니다.  
`Ok` variant 는 작업이 성공적으로 완료되었음을 의미하고, `Err` variant 는 작업이 실패했음을 의미합니다.  
`Result` 객체는 또한 `expect` 함수를 가지고 있습니다.  
`Result` 가 `Err` 값을 가진다면 `expect` 함수는 인자 값으로 전달된 값을 출력합니다.  
`Result` 가 `Ok` 값을 가진다면 `expect` 함수는 `Ok` 값을 반환합니다.  

만약 `epect` 함수를 사용하지 않는다면 warning 이 발생합니다.  

## Printing Values with println! Placeholders

`println!` 매크로는 문자열을 출력할 때 사용합니다.  

```rust
    println!("You guessed: {guess}");
```

`println!` 매크로는 `{}` 를 사용하여 문자열에 값을 삽입할 수 있습니다.  
`{}` 를 _placeholder_ 라고 부릅니다.  
expression 의 결과를 바로 출력하고 싶다면 빈 중괄호를 넣고 쉼표로 구분된 expression 을 넣으면 됩니다.  

```rust
let x = 5;
let y = 10;

println!("x = {x} and y + 2 = {}", y + 2);
```

---

* [목차로](../../README.md)
