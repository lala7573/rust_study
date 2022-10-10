---
uuid: 202210091531
tags: rust
link: 
aliases: 
  - 04_memory_ownership 
---

# 04_memory_ownership
러스트 프로그램의 메모리 안정성을 컴파일 타임에 보장할 수 있음

- 소유권이 뭔가요?
  - 스택과 힙
    - 러스트와 같은 시스템 프로그래밍 언어에서는 값이 스택/힙에 있는지 여부가 언어의 동작방식과 결정에 더 큰 영향을 줌
    - 스택
      - 빠름
        - 데이터를 넣어두기 위한 공간 혹은 데이터를 가져올 공간을 검색할 필요가 없음
        - 스택에 담긴 모든 데이터가 결정되어있는 고정된 크기를 갖고 있음
    - 힙
      - 컴파일 타임에 크기가 결정되지 않거나 크기가 변경될 수 있는 데이터 저장
      - allocation on the heap (allocating)
        - 프로그램은 OS에 저장할 공간 물어보고, OS는 충분히 커다란 힙 안의 빈 어떤 지점을 찾아서 이곳을 사용중이라고 표시하고 포인터 리턴
        - 큰 공간을 할당 받을 때 시간이 걸릴 수 있음
      - 스택보다 데이터 접근이 느림
        - 포인터가 가리킨 곳을 따라가야하기 때문
        - 메모리 내부를 덜 뛰어다닐 때 더 빨라짐

  - 소유권 규칙
  ```
  1. 러스트의 각각의 값은 해당값의 오너(owner)라고 불리우는 변수를 갖고 있다.
  2. 한번에 딱 하나의 오너만 존재할 수 있다.
  3. 오너가 스코프 밖으로 벗어나는 때, 값은 버려진다(dropped).
  ```
  - 메모리와 할당
    - string literal: 컴파일타임에 텍스트가 최종 실행 파일에 하드코딩됨
    - String: 변경 가능하고 커질 수 있는 텍스트 지원하기 위한 타입. 힙에 공간 할당받아 저장해야함
      - 스코프를 벗어날 때 러스트는 `drop` 라는 함수를 호출 함. `}`가 닫힐때 자동적으로 `drop`을 호출함 (C++에서는 Resource Acquisition Is Initialization, RAII) 라고 부름
    - `move`
      ```
      let s1 = String::from("hello");
      let s2 = s1;
      ``
      - 얕은 복사와 비슷. 러스트는 첫번째 변수를 무효화 시키기도 하기 때문에 move라고 부름
      - 실제로 String의 경우 ptr, len, capacity가 복사됨. 깊은 복사를 만들지 않음. 그러므로 어떠한 자동적인 복사라도 런타임 실행과정에서 효율적이라 가정할 수 있음
      - double free error로 인해 memorry corruption이 일어날 수 있음. 이는 보안 취약성 문제를 일으킬 가능성
    - `clone`
      ```
      let s1 = String::from("hello");
      let s2 = s1.clone();
      ```
      - 깊은 복사를 원할 때 `clone` 이라고 부르는 공용 메소드를 사용할 수 있음. 비용이 많이 들어갈 수 있음
    - `copy`
      - u32, bool, f64, Copy가 가능한 타입만으로 구성된 튜플들
      - `Copy` 트레잇 어노테이션 
        - 정수형과 같이 스택에 저장할 수 있는 타입에 대해 달 수 있는 어노테이션
        - `Copy` 트레잇을 갖고 있다면, 대입 과정 후에도 예전 변수를 계속 사용할 수 있음. 
        - 타입이 `Drop` 트레잇을 구현한 것이 있다면 `Copy` 트레잇을 어노테이션 할 수 없게 함
        - `Copy`어노테이션을 타입에 어떤식으로 추가하고싶은지 알고싶다면 부록 C의 Derivable Traits 참조

- 참조자외 빌림
  - 참조자 규칙
    1. 아래 중 둘 중 하나만 가질 수 있음
      - 하나의 가변 참조자
      - 임의 개수의 불변 참조자
    2. 참조자는 항상 유효해야만 한다
  - 참조자(reference, &)
    - 값의 소유권을 넘기는 대신 개체에 대한 참조자를 인자로 사용하면 값을 소유하지 않고 참조할 수 있게 해줌
  - 가변 참조자(mutable reference, &mut)
    - 특정한 스코프 내에 특정한 데이터 조각에 대한 가변 참조자를 딱 하나만 만들 수 있음
    ```
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s; // cannot borrow `s` as mutable more than once at a time
    ```
    - 러스트가 컴파일 타임에 데이터 레이스를 방지할 수 있게 해줌
    - 데이터 레이스는 정의되지 않은 동작을 일으키고 런타임에 추적&진단이 어려워 고치기 어려움. 러스트는 컴파일이 안되게 하는 방식으로 이 문제를 막음
      ```
      1. 두 개 이상의 포인터가 동시에 같은 데이터에 접근한다.
      2. 그 중 적어도 하나의 포인터가 데이터를 쓴다.
      3. 데이터에 접근하는데 동기화를 하는 어떠한 메커니즘도 없다.
      ```
    - 불변 참조자가 있을 때도 가변 참조자를 만들 수 없음. 불변 참조자의 사용자는 사용중인 동안에 값이 바뀌리라 예상하지 않음
    ```
    let mut s = String::from("hello");

    let r1 = &s; // 문제 없음
    let r2 = &s; // 문제 없음
    let r3 = &mut s; // 큰 문제
    ```
  - 댕글링 참조자(Dangling References)
    - 댕글링 포인터: 어떤 메모리를 가리키는 포인터를 보존하는 동안, 그 메모리를 해제함으로써 다른 개체에게 사용하도록 줘버렸을 지도 모를 메모리를 참조하고 있는 포인터를 말함
    - 러스트는 모든 참조자들이 댕글링 참조자가 되지 않도록 보장해줌
    ```
    fn main() {
        let reference_to_nothing = dangle();
    }

    fn dangle() -> &String {
        let s = String::from("hello");

        &s
    }
    // missing lifetime specifier -> lifetime 개념은 10장에서 다룸
    // this function's return type contains a borrowed value, but there is no value for it to be borrowed from.
    ```
- 슬라이스(slices)
소유권을 갖지 않는 데이터 타입
컬렉션 전체가 아닌 컬렉션의 연속된 일련의 요소들을 참조할 수 있게함
```
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

스트링 리터럴은 슬라이스
```
let s = "Hello, world!"; 
// s는 &str
// 스트링 리터럴이 불변인 이유도 설명해줌
// &str 불변 참조자
```

```
fn first_word(s: &String) -> &str {} // 1
fn first_word(s: &str) -> &str {} // 2
// 2번 방식이 좋음. &String, &str 에 둘다 적용 가능
```
배열에도 적용 가능