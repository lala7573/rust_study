---
uuid: 202210270851
tags: rust
link: https://rustwiki.org/en/book/ch10-00-generics.html
aliases: 
  - 10_generic_trait_lifetime 
---

# 10_generic_trait_lifetime
## generics
- 러스트는 컴파일 타임에 제네릭을 사용하는 코드에 대해 단형성화(monomorphization) 를 수행함으로써 런타임 비용이 없음
- 단성형화
```
// Option 열거형을 사용하는 예제 
let integer = Some(5);
let float = Some(5.0);

// 컴파일러가 생성한 단형성화된 코드
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

## trait
- 트레잇 정의
```
pub trait Summarizable {
    fn author_summary(&self) -> String;

    fn summary(&self) -> String {
        format!("(Read more from {}...)", self.author_summary())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summarizable for NewsArticle {
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```
- `T: PartialOrd + Copy` 트레잇 바운드
```
use std::cmp::PartialOrd;

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];

    let result = largest(&numbers);
    println!("The largest number is {}", result);

    let chars = vec!['y', 'm', 'a', 'q'];

    let result = largest(&chars);
    println!("The largest char is {}", result);
}
```
- 트레잇 혹은 타입이 우리의 Crate 내의 것일 경우에만 해당 타입에서의 트레잇을 정의할 수 있음. 즉, 외부 타입에 대한 외부 트레잇 구현은 허용되지 않음
  - 부모 타입이 존재하지 않기 때문에 고아 규칙(orphan rule)이라 부름
  - 이 규칙이 없다면 동일 타입에 대한 동일 트레잇 구현이 가능해지고, 두 구현체가 충돌함
- 트레잇과 트레잇 바운드를 통해 컴파일러가 제네릭 타입이 어떤 동작을 할 필요가 있는지를 정확히 알 수 있음


## lifetime
- 라이프타임의 주목적은 댕글링 참조자(dangling reference)를 방지하는 것
- 라이프타임 생략 규칙 (lifetime elision rules)
  - 특정한 상황에서 똑같은 라이프타임 명시를 계속하여 타이핑 -> 예측 가능하며 결정론적 패턴을 따름 -> borrow checker가 라이프타임을 추론할 수 있도록 한 것 = 생략 규칙
  - 생략 규칙들은 모든 추론을 제공하지는 않고, 나중에 더 추가될 수 있음
  - 규칙 적용 후, 참조자들이 어떤 라이프 타임을 가지고 있는지 모호하다면, 컴파일러는 에러를 줌 (사용자가 lifetime 을 추가할 수 있도록 함)
1. 참조자인 각각의 파라미터는 고유한 라이프타임 파라미터를 갖습니다. 바꿔 말하면, 하나의 파라미터를 갖는 함수는 하나의 라이프타임 파라미터를 갖고: fn foo<'a>(x: &'a i32), 두 개의 파라미터를 갖는 함수는 두 개의 라이프타임 파라미터를 따로 갖고: fn foo<'a, 'b>(x: &'a i32, y: &'b i32), 이와 같은 식입니다.

2. 만일 정확히 딱 하나의 라이프타임 파라미터만 있다면, 그 라이프타임이 모든 출력 라이프타임 파라미터들에 대입됩니다: fn foo<'a>(x: &'a i32) -> &'a i32.

3. 만일 여러 개의 입력 라이프타임 파라미터가 있는데, 메소드라서 그중 하나가 &self 혹은 &mut self라고 한다면, self의 라이프타임이 모든 출력 라이프타임 파라미터에 대입됩니다. 

