---
uuid: 202211042212
tags: rust
link: https://rinthel.github.io/rust-lang-book-ko/ch12-03-improving-error-handling-and-modularity.html
aliases: 
  - 12_IOProject_CmdlineProgram 
---

# 12_IOProject_CmdlineProgram

## 바이너리 프로그램의 핵심 기능을 나누기 위한 가이드라인 프로세스
main.rs는 프로그램 실행을 담당하고, lib.rs는 맡은 작업에 관한 로직을 담당
1. 프로그램을 main.rs와 lib.rs로 나누고 프로그램의 로직을 lib.rs로 옮깁니다
2. 커맨드라인 파싱 로직이 크지 않으면, main.rs에 남겨둬도 됩니다.
3. 커맨드라인 파싱 로직이 복잡해지기 시작할 거 같으면, main.rs 에서 추출해서 lib.rs로 옮기세요.
4. 이런 절차를 통해 main 함수에는 다음의 핵심 기능들만 남아있어야 합니다
  - 인자 값들로 커맨드라인을 파싱하는 로직 호출
  - 다른 환경들 설정
  - lib.rs의 `run`함수 호출
  - `run`이 에러를 리턴하면, 에러처리

