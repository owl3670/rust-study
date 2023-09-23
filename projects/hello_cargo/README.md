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

## Building and Running a Cargo Project

Cargo 를 사용하여 프로젝트를 build 하고 실행하는 명령어는 다음과 같습니다.  

```bash
$ cargo build
```

위의 명령어를 실행하면 _target/debug_ 디렉토리에 _hello_cargo_ 라는 이름의 실행파일이 생성됩니다.  
default build 는 debug build 입니다.  

`cargo build` 를 처음 실행하면 _Cargo.lock_ 파일도 생성됩니다.  
이 파일은 프로젝트가 의존하는 crate 의 정확한 버전을 저장합니다.  

compile 과 함께 실행파일을 실행하고 싶다면 다음과 같이 입력할 수 있습니다.  

```bash
$ cargo run
```

`cargo run` 이 `cargo build` 에 비해 더 간편하고 기억하기 쉬워 많은 개발자들이 사용합니다.  

`cargo run` 은 source file 이 수정되었거나 처음 실행때만 compile 을 하고 실행파일을 실행합니다.  
만약 source file 이 변경되지 않았을 때 `cargo run` 을 실행하면 compile 을 생략합니다.  

Cargo 는 `cargo check` 라는 명령어를 통해 빠르게 code 를 검사할 수 있습니다.  

```bash
$ cargo check
```

`cargo check`는 실행파일을 만들지 않기 때문에 빠르게 동작합니다.  
그래서 code 가 compile 가능한지 여부만 빠르게 판단할 때 사용할 수 있습니다.  

## Building for Release

project 를 release 할 준비가 되면 `cargo build --release` 명령어를 사용하여 release build 를 할 수 있습니다.  
이 명령어를 사용하면 _target/debug_ 디렉토리가 아닌 _target/release_ 디렉토리에 실행파일이 생성됩니다.  
release build 는 debug build 보다 더 오래 걸리지만 최적화를 하기에 실행파일의 성능이 더 좋습니다.  

## Cargo as Convention

Rust 개발자들은 Cargo 를 사용하여 프로젝트를 관리하는 것을 권장합니다.  
Cargo 를 사용하면 프로젝트의 의존성을 쉽게 관리할 수 있고, build, test, run 등의 명령어를 쉽게 사용할 수 있습니다.

---

* [목차로](../../README.md)
