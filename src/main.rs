use assignment_7_q3_lib::arithmetic::division;

fn main() {
   let result= division::division_res(20.0,4.0);
   match result{
        Some(a)=>println!("{:.2}",a ),
        None=> println!("Cannot divided by zero"),

   }
}
