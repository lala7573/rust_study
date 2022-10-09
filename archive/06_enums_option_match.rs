

// enum IpAddrKind {
//     V4,
//     V6,
// }
// struct IpAddr1 {
//     kind: IpAddrKind,
//     address: String,
// }

// let home = IpAddr1 {
//     kind: IpAddrKind::V4,
//     address: String::from("127.0.0.1"),
// };

// let loopback = IpAddr1 {
//     kind: IpAddrKind::V6,
//     address: String::from("::1"),
// };

// enum IpAddr {
//     V4(String),
//     V6(String),
// }

// let home = IpAddr::V4(String::from("127.0.0.1"));
// let loopback = IpAddr::V6(String::from("::1"));

// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// let home = IpAddr::V4(127, 0, 0, 1);

// let loopback = IpAddr::V6(String::from("::1"));
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
// struct QuitMessage; // 유닛 구조체
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String); // 튜플 구조체
// struct ChangeColorMessage(i32, i32, i32); // 튜플 구조체

// impl Message {
//     fn call(&self) { // 열거형도 impl 을 선언할 수 있음
//         // 메소드 내용은 여기 정의할 수 있습니다.
//     }
// }
// fn main() {
//     let m = Message::Write(String::from("hello"));
//     m.call();
// }
fn main() {

    println!("Hello, world!");
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;
}
