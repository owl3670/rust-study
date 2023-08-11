# Defining an Enum

struct 는 연관있는 field 와 data 를 함께 묶을 수 있는 방법을 제공했습니다.  
enum 은 value 가 특정한 value 집합 중 하나라는 것을 말할 수 있는 방법을 제공합니다.  
예를 들어 사과는 귤, 포도, 배와 같은 과일의 집합 중 하나라고 말할 수 있습니다.  

IP Address 는 V4 와 V6 가 있는데 IP 주소 표현 방법은 둘 중 하나이여야 하지 둘 다 일수는 없습니다.  
이러한 성격의 데이터 구조를 표현할 때 enum 이 유용합니다.  

```rust
enum IpAddrKind {
    V4,
    V6,
}
```