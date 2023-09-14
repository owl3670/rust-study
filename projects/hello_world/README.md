# Hello, World!

새로운 프로그래밍 언어를 습득할 때 전통적으로 가장 먼저 하는 것은 `Hello, World!`를 출력하는 것입니다.  
Rust 를 사용해서 `Hello, World!` 를 출력해보면서 Rust 학습을 시작해 보겠습니다.

## Creating a Project Directory

제일 먼저 Rust code 를 저장할 directory 를 만들어야 합니다.  
Rust 는 code 가 어디에 있는지는 상관하지 않지만 code 를 한 곳에서 쉽게 관리하기 위해서는 project 별로 directory 를 만드는 것이 좋습니다.  

terminal 을 통해 directory 를 만드는 것은 아래와 같이 하시면 됩니다.  

Linux, macOS, PowerShell 환경  

```bash
$ mkdir ~/projects
$ cd ~/projects
$ mkdir hello_world
$ cd hello_world
```

Windows CMD 환경

```bash
> mkdir "%USERPROFILE%\projects"
> cd /d "%USERPROFILE%\projects"
> mkdir hello_world
> cd hello_world
```

## Writing and Running a Rust Program

다음으로 생성한 directory 에서 _main.rs_ 라는 이름의 source 파일을 만들어야 합니다.  
Rust file 은 언제나 _.rs_ 확장자로 끝나게 됩니다.  
만약 파일 이름을 두 단어 이상 사용해야 한다면 convention 을 따른다면 각 단어를 소문자로 하여 underscore(_) 로 이어줘야 합니다.  

이제 _main.rs_ 파일을 열어 code 를 작성해 봅시다.  

```rust
fn main() {
    println!("Hello, world!");
}
```

파일을 저장하고 terminal 창을 열어 파일이 있는 디렉토리에서 다음과 같이 명령어를 입력하여 compile 과 실행을 할 수 있습니다.  


Linux, macOS 환경

```bash
$ rustc main.rs
$ ./main
Hello, world!
```

Windows 환경

```bash
> rustc main.rs
> .\main.exe
Hello, world!
```

운영체제에 상관없이 `Hello, world!` 문자열이 출력되는 것을 확인 할 수 있습니다.

## Anatomy of a Rust Program

위에서 작성한 "Hello, world!" program 을 자세히 보겠습니다.  

```rust
fn main() {
    
}
```

먼저 위의 code 는 `main` 이라는 이름의 함수를 정의하는 것입니다.  
`main` 은 특별한 함수로 실행가능한 Rust program 에서는 항상 첫 번째로 실행시키는 함수입니다.  
`main()` 에서 `()` 는 parameter 를 받기 위한 용도입니다.  
parameter 는 함수에 전달해야하는 값을 말합니다.  
위의 `main()` 함수에서는 아무것도 받을 것이 없다는 것을 말합니다.  

```rust
println!("Hello, world!");
```

`main` 함수 안에서 저러한 코드를 볼 수 있는데 이는 `println!` 이라는 Rust macro 를 호출하는 코드입니다.  
macro 가 아닌 함수를 호출하고자 한다면 `!` 를 제거하면 됩니다.  
macro 에 대해서는 나중에 얘기될 것입니다.  
`println!()` 의 `()` 안으로 `"Hello, world!"` 라는 문자열이 전달되고 있습니다.  
이렇게 전달되는 값을 argument 라 합니다.  
parameter 는 어떠한 값을 받을 것인지 미리 정의하는 것이라면 argument 는 실제로 전달되는 값이라고 보시면 됩니다.  
위의 코드 마지막 글자로 세미콜론 (`;`)이 들어가는 것을 볼 수 있습니다.  
이는 expression 의 끝을 의미하는 것으로 즉 다음 expression 시작과 구분짓는 용도입니다.  
Rust code 의 대부분의 끝 글자는 세미콜론으로 마무리 됩니다.  
