---
uuid: 202210151753
tags: rust
link: https://rustwiki.org/en/book/ch09-00-error-handling.html
aliases: 
  - 10_error_handling 
---

# 10_error_handling
## unrecoverable error
- 어떤 종류의 버그가 발견되었고 프로그래머가 이 에러를 어떻게 처리할지가 명확하지 않을 때
  - e.g. 배열의 끝을 넘어선 위치의 값에 접근하려고 시도하는 경우
- `panic!`
  -  매크로 실행 시
    1. 실패 메세지를 출력하고
    2. 스택 unwinding (되감기) & 제이터 제거
      - unwinding 시 각 함수로부터 스택을 거꾸로 훑어가면서 데이터를 제거하는데, 대신 abort 할 수 있음
        ```
        [profile.release]
        panic = 'abort'
        ```
    4. 종료
- `panic!` backtrace 사용하기
  ```
  // RUST_BACKTRACE=1 cargo run
  fn main() {
      let v = vec![1, 2, 3];
      v[99];
  }
  ```
  - `cargo build` 혹은 `cargo run`을 `--release` 플래그 없이 사용하는 경우 debug symbol이 기본적으로 켜짐

## recoverable error
- 복구 가능한 에러는 사용자에게 문제를 보고하고 연산을 재시도하는 것이 보통 합리적인 경우 
  - e.g. 파일을 찾지 못하는 에러
- `Result<T, E>` 

```
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        "Tried to create file but there was a problem: {:?}",
                        e
                    )
                },
            }
        },
        Err(error) => {
            panic!(
                "There was a problem opening the file: {:?}",
                error
            )
        },
    };
}
```

- unwrap

```
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
}
```

- expect

```
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
```

- 에러 전파하기

```
se std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

- 에러 전파를 위한 숏컷: ?
  - ?는 Result를 반환하는 함수에서만 사용될 수 있음

```
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
```

- panic! or not

```
pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}
```
