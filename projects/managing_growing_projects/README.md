거대한 프로그램을 만들면서 코드를 정리하는 것은 점점 더 중요해 집니다.  
프로젝트가 성장함에 따라 코드를 여러 모듈로 나누고 여러 파일로 분리하는 것이 좋습니다.  

이 장에서는 다음과 같은 것들을 다룰 것입니다.  

Packages: build, test, crate을 공유할 수 있게 해주는 Cargo의 기능  
Crates: 라이브러리 또는 실행 파일을 생성하는 모듈 트리  
Modules and use: 조직, 범위 및 개인 정보 경로를 제어할 수 있습니다.  
Paths: struct, function 또는 module과 같은 항목의 이름 지정 방법  

# Packages and Crates

Crate 는 Rust 컴파일러가 고려하는 최소한의 코드 양입니다. 단일 소스 코드를 실행하더라도 컴파일러는 그 코드를 하나의 crate 로 간주합니다.  
Crate 는 module 을 포함 할 수 있습니다. module 은 다른 crate 와 함께 컴파일되는 다른 파일에 정의될 수 있습니다.  

Crate 는 라이브러리 crate 또는 binary crate 로 분류됩니다.  
binary crate 는 실행 파일을 생성하는 crate 입니다. `main` 함수를 항상 포함해야 합니다.  

라이브러리 crate 는 `main` 함수를 가지지 않습니다.  
라이브러리 crate 는 단독으로 실행할 수 없지만 다른 프로젝트 등에서 공통으로 사용할 수 있는 기능들이 정의 됩니다.

package 는 하나 이상의 crate 번들입니다.  
package 는 `Cargo.toml` 파일을 포함하고 있으며 `Cargo.toml` 파일은 crate 들을 어떻게 빌드 할 것인지 설명합니다.  
Cargo 또한 command-line 도구를 위한 binary crate 를 포함하고 있는 package 입니다.  
package 에는 binary crate 는 원하는 만큼 포함할 수 있지만 library crate 는 오직 하나만 가능합니다.  
package 에는 어떤 crate 든 하나 이상이 포함되어야 합니다.  

# Defining Modules to Control Scope and Privacy

## Modules Cheat Sheet

- **Start from the crate root**: crate를 컴파일할 때 컴파일러는 처음으로 crate root 파일(src/lib.rs, src/main.rs 등의 파일이 보통 사용됨)을 찾게됩니다.
- **Declaring modules**: crate root 파일 안에서 새로운 module 을 선언할 수 있습니다. module 은 `mod` 키워드를 사용하여 선언합니다. 만약 `mod garden`으로 garden 모듈을 선언하면 컴파일러는 다음과 같은 곳에서 module 의 코드를 찾게 됩니다.
  - inline 으로 `mod garden` 다음 중괄호 안
  - `src/garden.rs` 파일
  - `src/garden/mod.rs` 파일
- **Declaring submodules**: crate root 파일이 아닌 다른 어떤 파일에서든 submodule 을 선언할 수 있습니다. 역시 `mod` 키워드를 사용합니다. 만약 `mod vegetables` 를 __src/garden.rs__ 파일에 선언하면 컴파일러는 다음과 같은 곳에서 submodule 의 코드를 찾게 됩니다.
  - inline 으로 `mod vegetables` 다음 중괄호 안
  - `src/garden/vegetables.rs` 파일
  - `src/garden/vegetables/mod.rs` 파일
- **Paths to code in modules**: module 이 crate 의 일부가 되었다면 보호 규칙이 허용하는 한 해당 module의 코드를 같은 crate 어디서든 참조 할 수 있습니다.
- **Private vs public**: module 내의 코드는 기본적으로는 상위 module로 부터는 private 입니다 module 을 공개하려면 `pub` 키워드를 사용합니다. public module 내의 항목도 공개하려면 선언 앞에 `pub`을 사용합니다.
- **The `use` keyword: Scope 안에서 `use` 키워드를 사용하여 module 항목의 축약형을 만들 수 있습니다.

`backyard` 라는 이름의 binary crate를 만들어 보겠습니다. 위에서의 룰을 재현하기 위해 아래와 같은 구조로 directory 와 file들을 만듭니다.

```bash
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```

위의 구조에서 crate root 파일은 __src/main.rs__ 입니다.

__src/main.rs__ 안의 코드를 보겠습니다.

```rust
use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
```

`pub mod garden;` 문장은 컴파일러에게 __src/garden.rs__ 파일에서 garden module 코드를 찾으라고 알려줍니다.  

__src/garden.rs__ 파일을 보겠습니다.

```rust
pub mod vegetables;
```

`pub mod vegetables;` 문장은 컴파일러에게 __src/garden/vegetables.rs__ 파일의 코드도 찾으라고 알려줍니다.

__src/garden/vegetables.rs__ 파일을 보겠습니다.

```rust
#[derive(Debug)]
pub struct Asparagus {}
```

`pub struct Asparagus {}` 문장은 컴파일러에게 `Asparagus` struct 를 공개하라고 알려줍니다.

---

* [목차로](../../README.md)
