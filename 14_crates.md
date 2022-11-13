---
uuid: 202211131831
tags: rust
link: https://rinthel.github.io/rust-lang-book-ko/ch14-00-more-about-cargo.html
aliases: 
  - 14_crates 
---

# 14_crates

- [14_crates](#14_crates)
  - [14.1. 릴리즈 프로필을 이용해 빌드 커스터마이징하기](#141-릴리즈-프로필을-이용해-빌드-커스터마이징하기)
  - [14.2. Crates.io에 크레이트 배포하기](#142-cratesio에-크레이트-배포하기)
  - [14.3. Cargo 작업공간](#143-cargo-작업공간)
  - [14.4. cargo install을 이용해 Crates.io에서 바이너리 설치하기](#144-cargo-install을-이용해-cratesio에서-바이너리-설치하기)
  - [14.5. 커스텀 명령어로 Cargo 확장하기](#145-커스텀-명령어로-cargo-확장하기)

## 14.1. 릴리즈 프로필을 이용해 빌드 커스터마이징하기
- Cargo 는 두 메인 프로필을 가짐
  - dev profile
    - cargo build
  - release profile
    - cargo build --release
- Cargo.toml 파일에서 설정
  ```
  [profile.dev]
  opt-level = 0

  [profile.release]
  opt-level = 3
  ```
  - 더 많은 옵션은 [cargo book](https://doc.rust-lang.org/cargo/) 참조


## 14.2. Crates.io에 크레이트 배포하기
- 로그인
  - cargo login {TOKEN}
  - https://crates.io/me/ 에 로그인 한 후, TOKEN을 생성
  - 내부 (~/.cargo/credentials) 에 토큰 저장
- 배포하기
  - cargo publish
  - liscense 필드엔 라이센스 식별자 값(license identifier value) 
    - [Linux Foundation’s Software Package Data Exchange (SPDX)](https://spdx.org/licenses/) 에서 식별자 확인
    - SPDX 에 없는 라이센스를 사용하고 싶은 경우, 파일로 만들고 license-file 추가
  - 버전 관리는 낙장불입
    - [Semantic Versioning rules](https://semver.org/) 참고
- 버전 yanking
  - 새롭게 만들어지는 프로젝트들이 해당 버전을 종속성으로 추가할 수 없게 함
  - 어떤 코드도 삭제하지 않음
  - cargo yank --vers 1.0.1 # yanking
  - cargo yank --vers 1.0.1 --undo # yanking 취소


## 14.3. Cargo 작업공간
- 작업공간
  - 동일한 Cargo.lock과 출력 디렉토리를 공유하는 패키지들의 집합
- Cargo.toml
  ```
  [workspace]

  members = [
    "adder",
    "add-one",
  ]
  ```
- adder, add-one 프로젝트 생성
  ```
  cargo new --bin adder
  cargo new add-one
  ```
- 각 패키지 실행
  ```
  cargo run -p adder
  ```
- 외부 크레이트 의존성
  - 작업공간은 최상위에만 단 하나의 Cargo.lock파일을 가짐
  - 크레이트들이 모든 의존성의 같은 버전을 사용함을 보증
- 테스트
  - cargo test
  - cargo test -p add-one
- 배포하기
  - 크레이트는 각각 배포함 
    - `-p`, `--all`같은 플래그가 없음


## 14.4. cargo install을 이용해 Crates.io에서 바이너리 설치하기
- cargo install
  - 로컬에서 바이너리 크레이트를 설치 & 사용할 수 있게 함
  - 바이너리 타겟을 가진 패키지만 설치할 수 있음
  - `${CARGO_HOME:-$HOME/.cargo}/bin` 에 바이너리 저장됨

## 14.5. 커스텀 명령어로 Cargo 확장하기
- 커스텀 명령어

  - `$PATH` 내 어떤 바이너리의 이름이 cargo-something 이고 해당 파이너리가 Cargo의 보조 명령어 바이너리일 경우 `cargo something`라는 명령어를 이용해 실행 가능
  - cargo --list 실행할 때의 목록에도 포함됨
