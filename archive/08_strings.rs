fn main() {
    let v = vec![1, 2, 3];

    let mut v = Vec::new();
    // 어떤 변수에 대해 그 변수가 담고 있는 값이 변경될 수 있도록 하려면, mut 키워드를 사용하여 해당 변수를 가변으로 만들어 줄 필요가 있음
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{:?}", &v);

    {
        let v = vec![1, 2, 3, 4];
    } // 벡터가 드롭될 때 벡터의 내용물 또한 전부 드롭
    // 벡터가 가지고 있는 정수들이 모두 제거된다는 의미입니다. 이는 직관적인 것처럼 보일 수도 있겠지만 벡터의 요소들에 대한 참조자를 만들때는 좀 더 복잡해 질 수 있음
    
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    //            - immutable borrow occurs here
    // v.push(6);
    //^^^^^^^^^ mutable borrow occurs here

    // println!("The first element is: {}", first);
}
