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