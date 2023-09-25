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

---

* [목차로](../../README.md)
