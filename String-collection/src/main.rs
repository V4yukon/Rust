fn main() {

    //let s = data.to_string();


    //let m = "initial contents";


    //println!("123{:?}", data);

    //println!("{}", std::any::type_name::<type_of(data)>());


   // println!("Hello, world!");
   let mut s = String::from("foo");


   s.push_str("tball");


   println!("{s}");


   let h = String::from("hell");

   let w = String::from("world!");


   let m = h + &w;

   //println!("{h}");

   println!("{w}");

   println!("{m}");

   let my_string = "345helloo world";

   let my_slice = &my_string[0..4];
   println!("{my_slice}");


   for c in "56&".bytes(){
       println!("{c}");
   }
   for l in "56&".chars(){
       println!("{l}");
   }
}
