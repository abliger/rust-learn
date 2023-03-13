fn main() {
    let number = 10_000;
    // let number : i8 = 300;
    let x = (1.0,1,"str",true);
    println!("{} - {} - {} - {}",x.0,x.1,x.2,x.3);
    let arr = [1,2,3,4,5];
    let arr2 = [3;5];
    println!("{} {}",arr[0],arr[4]);
    println!("{} {}",arr2[0],arr2[4]);
}
