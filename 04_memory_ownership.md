---
uuid: 202210091531
tags: rust
link: 
aliases: 
  - 04_memory_ownership 
---

# 04_memory_ownership
러스트 프로그램의 메모리 안정성을 컴파일 타임에 보장할 수 있음

## 소유권이 뭔가요?

### 소유권 규칙
```
1. 러스트의 각각의 값은 해당값의 오너(owner)라고 불리우는 변수를 갖고 있다.
2. 한번에 딱 하나의 오너만 존재할 수 있다.
3. 오너가 스코프 밖으로 벗어나는 때, 값은 버려진다(dropped).
```
## 참조자외 빌림

## 슬라이스(slices)
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