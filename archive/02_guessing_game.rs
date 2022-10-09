
/**

# config file: ~/.cargo/config.toml
[net]
git-fetch-with-cli = true 
 */


extern crate rand;
// == use rand;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); 
    // rand::thread_rng 운영체제에 의해 시드되는 난수 생성기
    // cargo doc --open

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess) // 참조자를 가변으로 바꾸기 위해 &guess 가 아니라 &mut guess
            .expect("Failed to read line"); 
          // io:Result 타입은 열거형임. 열거형은 정해진 값들만 가질 수 있고 이러한 값들은 열거형의 Variant라고 부름. Variants를 Ok와 Err로 가짐. 
          // expect 를 호출하지 않는다면 컴파일은 되지만 경고가 나타남

        let guess: u32 = match guess.trim().parse() { // parse 사용시 정확한 정수 타입 명시
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) { // std::cmp::Ordering 도 열거형. Less, Greater, Equal
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
