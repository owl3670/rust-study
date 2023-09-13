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