/*
 * It is possible to declare first and initialize later.
 * However, this may lead to unitialized variables and compile forbids it
 */

 fn main() {
     let later_initialized;

     // I can't use later_initialized right now, unless I initialize it.
     // i.e the below line will be an error
     // println!("later initialized {}", later_initialized);

     later_initialized = 42;
     println!("later initialized {}", later_initialized);
 }