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


---

* [목차로](../../README.md)
