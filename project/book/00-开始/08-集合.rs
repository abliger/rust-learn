use std::collections::HashMap;
fn main() {
   let mut vec = Vec::new();
   vec.push(1);
   vec.push(2);
   vec.push(3);
   vec.push(4);
   vec.push(5);
   let first = &vec[0];
   for i in &vec {
       println!("{i}");
   }
   for i in &mut vec{
      *i = *i + 2;
   }
   println!("============");
   vec.push(0);
   let first = &vec[0];
   println!("first is {}",first);
   for i in vec {
       println!("{i}");
   }

   let str = "str".to_string();

   let str = str + "test21";
   // let str = "befor" + str;
   println!("{str}");

   let mut map = HashMap::new();

   map.insert("test1",1);
   map.insert("test2",2);
   map.insert("test3",3);
   for (k,v) in &mut map{
     *v = 10;
   }
   for (k,v) in map{
       println!("{k}-{v}",);
   }

}
