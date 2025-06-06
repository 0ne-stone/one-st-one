fn main() {

    // std::mem::size_of::<Type>()은 타입의 크기를 바이트 단위로 제공합니다.
    println!("A String is always {:?} bytes. It is Sized.", std::mem::size_of::<String>());
    println!("A i8 is always {:?} bytes. It is Sized.", std::mem::size_of::<i8>());
    println!("A f64 is always {:?} bytes. It is Sized.", std::mem::size_of::<f64>());

    // std::mem::size_of_val()은 변수의 바이트 크기를 제공합니다. 
    println!("But a &str? It can be anything. '양원석' is {:?} bytes. It is not Sized.", std::mem::size_of_val("양원석"));
}
