fn main() {
let a = 4;
if a == 1{
    println!("a 是 1");
} else if a ==2 {
    println!("a is 2");
} else if a == 4 {
    println!("a is 4");
}else {
    println!("没有匹配成功");
}
let a = if a > 5 {
    100
}else{
    0
};
println!("a is {a}");

let g =(true,"str");
if let  (_,"sss") =g {
    println!("g2 is sss");
} else if let (g1,"str") = g {
 println!("g1 is {g1}");
};

let mut b =0;
loop {
  if b >10{
      break;
  }  
  println!("a is {a}");
  b +=2;
}
println!("down");

while b < 20{
    b+=1;
    println!("b is {b}");
}

for i in 0..10{
    println!("{i}");
}
}
