# Hello, Cargo!

Cargo 는 Rust 의 build system 이고 package manager 입니다.  
만약 이전의 Hello, World! 프로젝트와 같은 단순한 프로젝트가 아니라 복잡한 코드의 Rust 프로그램을 만든다면 의존성을 추가해야 할 수 있습니다.  
이때 Cargo 를 사용한다면 의존성 추가를 더 쉽게 할 수 있습니다.  

공식 설치파일로 Rust 를 설치했다면 Cargo 는 이미 설치 되어 있습니다.  
Cargo 가 설치 되었는지 버전을 확인하는 명령어는 다음과 같습니다.  

```bash
$ cargo --version
```

## Creating a Project with Cargo

```bash
$ cargo new hello_cargo
$ cd hello_cargo
```

위의 명령어를 실행하면 Cargo 는 _hello_cargo_ 라는 이름의 디렉토리를 생성하고 _hello_cargo_ 디렉토리로 이동합니다.  
그리고 _hello_cargo_ 디렉토리에는 _Cargo.toml_ 이라는 파일과 _src_ 라는 _main.rs_ 파일이 있는 디렉토리가 생성됩니다.  

_Cargo.toml_ 파일은 Cargo 의 설정 파일입니다.  

```
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

[dependencies]
```

_Cargo.toml_ 파일의 `[package]` 섹션은 Rust 프로젝트의 metadata 를 저장합니다.  
`[dependencies]` 섹션은 프로젝트가 의존하는 crate 의 이름과 버전을 저장합니다.  

_main.rs_ 파일을 열어보면 다음과 같은 코드가 있습니다.  

```rust
fn main() {
    println!("Hello, world!");
}
```

---

* [목차로](../../README.md)
