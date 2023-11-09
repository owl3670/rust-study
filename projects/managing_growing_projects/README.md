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

## Grouping Related Code in Modules

module 을 사용하면 가독성을 높이고 쉽게 재사용할 수 있도록 crate 안의 코드를 정리할 수 있습니다. 또한 모듈내의 코드는 기본적으로 비공개이므로 개인 정보를 제어할 수 있습니다.  
레스토랑을 구현하는 코드를 생각해보겠습니다. 레스토랑은 고객들에게 보여지는 곳을 front of house, 셰프와 요리사, 관리자가 업무를 수행하는 back of house로 구분 할 수 있습니다.  
이러한 방식으로 crate 를 구성하기 위해 기능을 중첩된 모듈로 구설 할 수 있습니다.  
`cargo new restaurant --lib` 를 실행하여 `restaurant`이라는 library 를 만들고 sr/lib.rf 에 다음과 같은 모듈과 함수들을 정의해보겠습니다.  

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```

위와 같이 모듈 내부에 다른 모듈을 배치할 수 있고 crate root 로 부터 모듈 트리를 만들 수 있습니다.  
이러한 모듈을 사용하면 관련 정의를 함께 그룹화하고 관련 이유를 쉽게 설명할 수 있습니다. 

# Paths for Referring to an Item in the Module Tree

Rust 에게 module tree 어디에서 item 을 가져올지 알려주기 위해 filesystem 과 같은 path 를 사용할 수 있습니다.  
path 는 두 가지 형태를 갖습니다.

- 절대 경로로 crate root 로부터 이어지는 full path 입니다. crate 이름으로 시작하며 현재 crate 로부터 시작되는 code 는 `crate` 로 시작할 수 있습니다.
- 상대 경로로 현재 모듈로 부터 시작되는 경로입니다. `self`, `super` 혹은 식별자를 사용합니다.
어떤 형태이든 path 는 `::` 를 사용하여 식별자를 구분합니다.

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

위의 코드는 절대경로와 상대경로로 함수를 호출하는 것을 보여줍니다.  
실제로는 module 의 item 이 공개상태가 아니기에 실행시 에러가 발생하지만 경로 자체는 유효합니다.  

## Exposing Paths with the `pub` Keyword

`pub` 키워드를 사용하여 module 의 item 을 공개할 수 있습니다.  
이전 예제에서 `hosting` module 의 `add_to_waitlist` 함수를 공개하려면 `pub` 키워드를 사용하여야 합니다.  

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

모듈을 공개한다고 해서 모듈 내의 모든 아이템을 공개하는 것은 아니기에 공개할 아이템 앞에 명시적으로 `pub` 키워드를 사용해야 합니다.  
이러한 보호 규칙은 모듈뿐만 아니라 구조체, 열거형, 함수, 메소드 등에도 적용됩니다.  

위의 코드에서 `front_of_house` module 은 공개되지 않았지만 `eat_at_restaurant` 함수와 같은 모듈에 정의되어 있으므로 `eat_at_restaurant` 함수는 `front_of_house` module 을 참조할 수 있습니다.  

다른 프로젝트에서 코드를 사용할 수 있도록 library crate를 공유하려는 경우 공개 API 로 사용자와 코드가 상호 작용할 수 있는 방법을 결정할 수 있습니다.  
이러한 API 의 변경 사항을 관리할 때 고려해야 할 사항이 많습니다. 이러한 주제에 관심이 있다면 [The Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) 를 참고하시기 바랍니다.  

## Starting Relative Paths with `super`

경로의 시작 부분에 `super` 를 사용하여 상위 모듈로 부터 시작하는 상대 경로를 만들 수 있습니다.  

```rust
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
```

## Making Structs and Enums Public

`pub` 키워드를 사용하여 struct 나 enum 을 공개할 수 있습니다.

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}
```

`toast` field 는 `pub` 키워드가 있어 `eat_at_restaurant` 함수에서 사용할 수 있지만 `seasonal_fruit` field 는 `pub` 키워드가 없기에 `eat_at_restaurant` 함수에서는 사용할 수 없습니다.  
만약 위의 코드에서 `meal.seasonal_fruit = String::from("blueberries");` 코드의 주석을 풀면 에러가 발생하게 됩니다.  

또하나 살펴봐야 할 점은 `back_of_house::Breakfast::summer` 함수처럼 `pub` 으로 공개 되어 인스턴스를 구성하여 제공하는 함수가 없다면  
`Breakfast` 에서 private field 가 있기에 `Breakfast` 인스턴스를 생성할 수 없다는 것입니다.  

`enum` 에서 `pub` 키워드를 사용하는 것을 보겠습니다.  

```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
        let order1 = back_of_house::Appetizer::Soup;
        let order2 = back_of_house::Appetizer::Salad;
    }
```

`enum` 에서는 그 variant를 사용하지 못한다면 의미가 없습니다.  
때문에 `enum` 키워드 앞에마 `pub` 키워드를 달아주면 모든 variant를 공개할 수 있습니다.

# Bringing Paths into Scope with the use Keyword

함수 호출을 위해 path 들을 적는 것이 너무 반복적이고 불편할 수 있습니다.  
`use` keyword 를 사용해 이를 간단히 할 수 있습니다.  

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

위의 코드에서 `use` 를 사용함으로 `hosting` module 의 `add_to_waitlist` 함수를 호출하는 것이 간단해졌습니다.  
`use` 를 사용하는 것은 파일 시스템에서 symbolic link 를 만드것과 유사합니다.  

`use` 는 특정 scope 에서만 유효합니다.  
만약 위의 코드에서 `eat_at_restaurant` 함수의 코드를 별도의 module 의 child 로 만든다면 컴파일 에러가 발생합니다.  

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

// 모듈로 감쌌기에 use 와 scope 가 분리됩니다.
mod customer{
  pub fn eat_at_restaurant() {
    hosting::add_to_waitlist(); // 컴파일 에러가 발생합니다.
  }
}
```

## Creating Idiomatic use Paths

`use` 를 이용해 `add_to_waitlist` 까지 경로를 추가하여 `hosting::add_to_waitlist` 대신 `add_to_waitlist` 만 호출해도 되도록 해보겠습니다.  

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
}
```

이렇게 함수를 사용하는 것은 편해 보이지만 `add_to_waitlist` 가 정의된 위치가 불분명합니다.  
함수를 호출할 때 상위 모듈을 지정하면 함수가 로컬에 정의되지 않았음을 명확히 합니다.  
Rust 에서는 함수의 부모 모듈을 사용 범위로 가져오는 것이 관용적인 방법입니다.  

한편으로 structs, enums, 기타 `use` 항목들은 전체 경로를 지정하는 것이 관용적입니다.  

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```

이러한 관용적인 표현의 예외는 이름이 같은 두 항목을 `use` 로 가져올때인데 이때는 상위 모듈까지만 기재되야 합니다.  

```rust
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}
```

---

* [목차로](../../README.md)
