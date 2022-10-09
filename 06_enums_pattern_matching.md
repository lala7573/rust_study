---
uuid: 202210091630
tags: rust
link: https://doc.rust-lang.org/book/ch06-00-enums.html
aliases: 
  - Enums and Pattern Matching
---

# Enums and Pattern Matching
- Enums
Haskell 대수 데이터 타입과 가장 비슷함
```
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
```
- `Option`
https://doc.rust-lang.org/std/option/enum.Option.html
```
enum Option<T> {
    Some(T),
    None,
}
let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;
```
- `match`
일련의 패턴에 대해 어떤 값을 비교한 뒤 어떤 패턴에 매치되었는지를 바탕으로 코드를 수행하도록 해주는 흐름 제어 연산자. 패턴은 리터럴 값, 변수명, 와일드 카드 등으로 구성될 수 있음.
```
#[derive(Debug)] // So we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // ... etc
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
          println!("Lucky penny!")
          1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
          println!("State quarter from {:?}!", state);
          25
        }
    }
}
```
  - Option<T>를 이용하는 패턴매칭
  러스트의 매치는 하나도 빠뜨리지 않음(exhaustive)
  ```
  fn plus_one(x: Option<i32>) -> Option<i32> {
      match x {
          None => None, 
          Some(i) => Some(i + 1),
      }
  }
  ```
  - _ placeholder
  ```
  let some_u8_value = 0u8;
  match some_u8_value {
      1 => println!("one"),
      3 => println!("three"),
      5 => println!("five"),
      7 => println!("seven"),
      _ => (),
  }
  ```
- `if let`
하나의 패턴만 매칭시키고 나머지 경우는 무시. 하나의 패턴에 매칭 될 때 다른 값에 대해서는 무시하는 `match`문을 위한 syntax sugar
`match`는 exhuastive 검사를 제공하고 `if let` 는 간결함을 제공함.
```
// 기존 패턴 매칭 문법
let some_u8_value = Some(0u8);
match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
}
```
```
// if let 적용 문법
if let Some(3) = some_u8_value {
    println!("three");
}
```