---
uuid: 202211132050
tags: rust
link: https://rinthel.github.io/rust-lang-book-ko/ch13-00-functional-features.html
aliases: 
  - 13_functional_features 
---

# 13_functional_features
## 13.1. 클로저: 환경을 캡쳐할 수 있는 익명 함수
- 클로저
  ```
  fn  add_one_v1   (x: u32) -> u32 { x + 1 }  // 함수
  let add_one_v2 = |x: u32| -> u32 { x + 1 };
  let add_one_v3 = |x|             { x + 1 };
  let add_one_v4 = |x|               x + 1  ;
  ```
  - 변수에 저장하거나 다른 함수에 인자로 넘길 수 있는 익명 함수
  - 프로그램에서 한곳에서 코드를 정의하고, 실제로 결과가 필요한 곳에서만 lazy하게 그 코드를 실행하게 해줌
  - 정의 할 때, 파라미터나 반환값의 타입을 명시하지 않아도 되지만 각 파라미터들과 그들의 반환값에 대해 단 하나의 추론된 구체적인 타입을 가짐
- 클로저를 이용한 Memoization(혹은 lazy evaluation) 패턴 구현하기
  ```
  #![allow(unused)]
  fn main() {
    use std::thread;
    use std::time::Duration;

    struct Cacher<T>
        where T: Fn(u32) -> u32
    {
        calculation: T,
        value: Option<u32>,
    }

    impl<T> Cacher<T>
        where T: Fn(u32) -> u32
    {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                },
            }
        }
    }

    fn generate_workout(intensity: u32, random_number: u32) {
        let mut expensive_result = Cacher::new(|num| {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            num
        });

        if intensity < 25 {
            println!(
                "Today, do {} pushups!",
                expensive_result.value(intensity)
            );
            println!(
                "Next, do {} situps!",
                expensive_result.value(intensity)
            );
        } else {
            if random_number == 3 {
                println!("Take a break today! Remember to stay hydrated!");
            } else {
                println!(
                    "Today, run for {} minutes!",
                    expensive_result.value(intensity)
                );
            }
        }
    }
  }
  ```
  - 각 클로저 인스턴스는 자신의 유일한 익명 타입을 가짐. 즉, 두 클로저가 동일한 타입 서명을 갖더라도 그들의 타입은 여전히 다른 것으로 간주 
  - 환경을 캡처(메모리 사용)해서 클로저가 정의된 스코프의 변수들을 접근
  - 3가지 방식으로 캡쳐
    - FnOnce 캡쳐한 변수를 소비함. 소유권을 클로저 안으로 가져옴. Once는 클로저가 동일한 변수들에 대해 한 번 이상 소유권을 얻을 수 없다는 것을 의미함(=한번만 호출 가능)
    - Fn 불변으로 빌려옴
    - FnMut 가변으로 빌려옴. 그 환경을 변경할 수 있음
  - 클로저가 환경을 캡쳐할 때 소유권을 갖도록 강제하고 싶으면 `move` 키워드를 사용함
    ```
    let equal_to_x = move |z| z == x;
    ```
## 13.2. 반복자로 일련의 항목들 처리하기
- iter()
  - `let mut v1_iter = v1.iter();`
  - 시퀀스의 어디에 있는지추적하기 위해 반복자가 사용하는 내부 상태를 변경(=반복자를 소비)
  - 불변 참조에 대한 반복자
- into_iter()
  - 소유권을 포함한 값에 대한 반복자
- iter_mut()
  - 가변 참조에 대한 반복자

- Iterator 트레잇 구현하기
  ```
  #![allow(unused)]
  fn main() {
      struct Counter {
          count: u32,
      }

      impl Iterator for Counter {
          type Item = u32;

          fn next(&mut self) -> Option<Self::Item> {
              self.count += 1;

              if self.count < 6 {
                  Some(self.count)
              } else {
                  None
              }
          }
      }
  }
  ```
  - next를 호출하는 메서드를 소비하는 어댑터들이라고 부름: sum
  - 반복자 어댑터 메서드: map, filter
    - collect와 함께 사용
  
## 13.3. I/O 프로젝트 개선하기
- greprs 프로젝트 참조

## 13.4. 성능 비교하기: 루프 vs. 반복자
- 반복자는 러스트의 제로 비용 추상화 중 하나이며, 그 추상을 사용하는 것은 추가 적인 실행시간 오버헤드가 없다
- Unrolling: 루프 제어 코드의 오버헤드를 제거하고 대신 루프의 각 순회에 해당하는 반복적인 코드를 생성하는 최적화 방법
  - 러스트는 12개의 순회가 있다는 것을 알고 있고, 어셈블리코드로 컴파일 될 때 어떤 루프도 없도록, 루프를 풀어(unroll) 둠
  ```
  let buffer: &mut [i32];
  let coefficients: [i64; 12];
  let qlp_shift: i16;

  for i in 12..buffer.len() {
      let prediction = coefficients.iter()
                                  .zip(&buffer[i - 12..i])
                                  .map(|(&c, &s)| c * s as i64)
                                  .sum::<i64>() >> qlp_shift;
      let delta = buffer[i];
      buffer[i] = prediction as i32 + delta;
  }
  ```
  - 모든 계수들은 레지스터에 저장되는데 값에 대한 접근이 매우 빠르다는 것을 뜻함
  - 실행시간에 배열 접근에 대한 경계 체크가 없음