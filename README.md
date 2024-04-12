# Rust-study

---

본 레포지토리는 Rust의 기초 문법을 공부하기 위해 만들어졌으며
Windows와 MacOS를 모두 사용한다.

- MacOS의 경우 설치 과정은 설명하지 않는다.

Rust 공식 홈페이지에 있는 인스톨러를 설치하고 C++ build tools를 설치한다.

이후 link.exe is not found와 같은 오류가 발생할 때는 아래 명령어를 따라가보자.

```
$ powershell
$ rustup toolchain install stable-x86_64-pc-windows-gnu
$ rustup default stable-x86_64-pc-windows-gnu
$ cargo run

```

현재 Path에 프로젝트를 구성하고 싶은 경우
**cargo init**

이외에 폴더(프로젝트)를 새로 구성하고 싶은 경우
**cargo new <Project Name>**
