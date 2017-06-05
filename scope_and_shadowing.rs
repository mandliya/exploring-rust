/*
 * This program displays use of scope in variable bindings in rust
 */

 fn main()
 {
     let outer_var = 42u32;

     // The below braces create a block.
     // Anything declared inside will have the scope of the block.
     // Anything declared before this scope is a valid in the scope of the block.
     {
        let inner_var = 34u32;
        println!("Inner variable is {}", inner_var);

        // The outer_var is valid here
        println!("Outer variable is {}", outer_var);

        // We can shadow outer_var with a new outer_var
        let outer_var = 5_f32;
        println!("Outer variable now is {}", outer_var);
     } // End of block

     // here inner varaible is no more valid.
     // outer variable is same as before block
     println!("Outer variable is {}", outer_var);

     // you can shadow outer variable again.
     let outer_var = 96u32;
     println!("Outer variable now is {}", outer_var);
 }