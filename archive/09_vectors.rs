use std::collections::HashMap;
//  세 가지 일반적인 컬렉션 중에서 이 해쉬맵이 제일 덜 자주 사용되는 것이기 때문에, 프렐루드(prelude) 내에 자동으로 가져와지는 기능에 포함되어 있지 않음
fn main() {
    let mut scores = HashMap::new();
    // 데이터를 힙에 저장

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    // 러스트는 벡터에 담긴 데이터의 타입에 기초하여 해쉬에 담길 타입을 추론

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("{:?}" , &map);

    // field_name과 field_value은 이 지점부터 유효하지 않습니다.
    // 이들을 이용하는 시도를 해보고 어떤 컴파일러 에러가 나오는지 보세요!
    // println!("{:?}", &field_name);
    //                  ^^^^^^^^^^^ value borrowed here after move

}
