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

## Testing the First Part of the Game

지금까지 작성한 코드를 실행해 보겠습니다.  

```bash
$ cargo run
```

# Generating a Secret Number

지금까지 작성한 코드는 user 의 input 을 받고 출력하는 기능을 가지고 있습니다.  
다음으로는 user 가 맞춰야 할 숫자를 랜덤으로 생성하는 기능을 추가해 보겠습니다.  

## Using a Crate to Get More Functionality

Rust 에서 crate 는 소스 코드 파일의 collection 입니다.  
`rand` crate 는 _library crate_ 이며, 다른 파일에서 사용되지 않고는 혼자서 실행될 수 없는 crate 입니다.  
cargo 에서 crate 를 관리하기 위해서 `Cargo.toml` 파일을 사용합니다.  
`Cargo.toml` 파일에 아래와 같이 `rand` crate 를 추가합니다.  

```toml
[dependencies]
rand = "0.8.5"
```

위에서 `rand` crate 의 버전을 0.8.5 로 지정했는데 이는 `^0.8.5` 의 줄임말입니다.  
이는 0.8.5 이상, 0.9.0 미만의 버전을 사용하겠다는 의미입니다.  

`Cargo.toml` 파일에 `rand` crate 를 추가한 후에는 `cargo build` 명령어를 실행하여 crate 를 다운로드 받고 빌드합니다.  

```bash
$ cargo build
```

cargo 는 [Crates.io](https://crates.io/) 에서 의존성이 있는 _registry_ 를 찾아서 다운로드 받습니다.  
_registry_ 가 업데이트 되면 cargo 는 아직 다운로드되지 않은 모든 crate 를 다운로드 합니다.
즉 cargo build 를 하고 나시면 의존성이 있는 다른 crate 들도 함께 다운로드 받고 빌드합니다.

### Ensuring Reproducible Builds with the _Cargo.lock_ File

프로젝트를 처음 빌드하면 cargo 는 `Cargo.lock` 파일을 생성합니다.  
이 파일은 프로젝트가 의존하는 crate 의 정확한 버전을 저장합니다.  
다음에 프로젝트를 다시 빌드한다면 `Cargo.lock` 파일을 확인하여 지정된 버전을 사용합니다.  
이렇게 하면 처음 빌드시의 환경과 동일한 버전의 crate 들로만 프로젝트를 빌드할 수 있습니다.

### Updating a Crate to Get a New Version

Crate 를 update 하고 싶다면 `update` 키워드를 사용해서 _Cargo.lock_ 파일을 무시하고 최신 crate 로 업데이트 할 수 있습니다.  
이때 버전 정책은 _Cargo.toml_ 에서 기입된 정보에 기반합니다.  
update 후에는 _Cargo.lock_ 파일에 새로운 버전들이 기입됩니다.  

## Generating a Random Number

`rand` crate 를 프로젝트에 포함하였으니 추측해야 할 번호를 랜덤하게 생성할 수 있게 되었습니다.  

```rust
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
```

먼저 `use rand::Rng;` 코드를 추가하여 랜던 번호 생성기가 구현되어 있는 `Rng` trait 을 사용할 수 있도록 합니다.  
이제 `rand::thread_rng` 함수를 통해 난수 생성기를 만들 수 있습니다.  
난수 생성기에서 `gen_range` 함수에 `1..=100` 범위 표현식을 전달하여 호출하게 되면 1 부터 100 사이의 숫자를 랜덤하게 생성할 수 있습니다.  

# Comparing the Guess to the Secret Number

이제 랜덤한 번호를 생성하고 유저의 input 을 받을 수 있기에 두 값을 비교해볼 수 있습니다.  

```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // --snip--

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```

먼저 우리는 비교를 위한 Type 을 표준 라이브러리에서 가져옵니다. `use std::cmp::Ordering;`  
`Ordering` type은 enum 으로 값의 비교에 대한 결과 값을 읽기 쉽게 합니다.  

`guess` 변수의 `cmp` 함수를 호출하면서 `&secret_number` 값을 인자로 넘기면 두 값의 비교 결과를 `Ordering` type 으로 반환 받을 수 있습니다.  

`match` 표현식은 주어진 값을 각 arm 의 패턴과 비교하여 값이 일치하는 arm 의 실행코드를 실행합니다.  
위에서 만약 `guess` 의 값이 `secret_number` 의 값보다 작다면 `"Too small!"` 이 출력될 것입니다.  

위의 코드를 실행하여 결과를 확인하려한다면 에러가 발생할 것이며, 이는 Rust 가 강한 정적 type 시스템이기 때문입니다.  
이전 단계에서 우리는 `guess` 변수를 만들때 `let mut guess = String::new()` 로 만들었습니다.  
즉 `guess` 의 type 은 `String` type 입니다. 
`secret_number` 는 number type 인데 별도로 type 을 지정하지 않았기에 default type 인 `i32` 입니다.  

우리는 이를 해결하기 위해 `String` type 을 변환해야 합니다.

```rust
    // --snip--

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
```

새로 추가된 line 은 아래와 같습니다.  

```rust
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

우리는 이미 위에서 `guess` 변수를 만들었는데 이래도 괜찮을까요?  
다행히 Rust 에서는 _Shadowing_ 이라는 개념이 있어서 이전의 변수를 새로운 변수가 가려줍니다.  

새로운 `guess` 변수는 `guess.trim().parse()` 의 결과 값과 bind 됩니다.  
`String` 객체의 `trim` 메서드는 시작과 끝에 있는 공백을 제거해줍니다.  
유저는 입력을 위해 엔터를 입력하는데 이때 이미 값과 함께 `\n` (Windows 에서는 `\r\n`) 이라는 "newline" 을 의미하는 문자도 포함됩니다.  
`trim` 메서드롤 통해 이러한 필요없는 값들을 제거합니다.  

`parse` 메서드는 string 에서 다른 type 으로 변환합니다.  
변수 선언부에서 `: u32` 를 통해 type 을 명확히 하면 해당 type 으로 자동으로 변환하게 됩니다.  

추가적으로 `u32` annotaion 을 통해 Rust 는 `guess` 값과 비교되는 `secret_number` 도 `u32` 여야 한다고 추론하게 됩니다.  

유저의 입력으로 숫자로 변환할 수 없는 값이 들어온다면 `expect` 메서드에서 주어진 문자열을 출력하고 종료됩니다.  

# Allowing Multiple Guesses with Looping

`loop` 를 사용하여 유저가 번호를 반복적으로 입력할 수 있게 만들 수 있습니다.

```rust
    // --snip--

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        // --snip--

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
}
```

이제 유저의 입력이 숫자가 아니여서 `parse` 에서 error 가 발생하거나 ctrl-c 로 프로그램을 종료할 때 까지 계속해서 숫자를 입력받을 수 있는 프로그램을 만들었습니다.

## Quitting After a Correct Guess

번호를 맞출때까지 입력을 무한히 받다가 번호를 맞췄을 때 `loop` 를 빠져나가고 프로그램을 종료하게 만들어 봅시다.

```rust
        // --snip--

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

이제 유저가 맞는 번호를 입력하면 `break` 를 통해 `loop` 를 빠져나가고 프로그램이 종료되게 됩니다.  



---

* [목차로](../../README.md)
