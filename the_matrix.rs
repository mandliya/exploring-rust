fn main() {

    // In Rust, Variable bindings can be type annotated when declared, however
    // powerful compiler of Rust can infer the types from the context.
    // For example see it in example from references of the movie "The Matrix"
    //
    let is_matrix_real = false;
    let is_neo_the_one = true;
    let truth = "There is no spoon.";
    let actual_year = 2199u32;
    let the_oracle = ();

    let year_in_matrix = actual_year - 1000;

    println!("Morpheus and Trinity believes Neo is the one. {}", if is_neo_the_one {"Yes"} else {"No"});
    println!("Matrix is not real: {}", if is_matrix_real {"Yes"} else {"No"});
    println!("Year in matrix is {} and actual year is {}", year_in_matrix, actual_year);
    println!("What is the truth? Asks Neo, '{}', replies the boy with spoon", truth);
    println!("The oracle is mysterious! and she is  {:?}", the_oracle);

    // The compiler warns about unused variable bindings; these warnings can
    // be silenced by prefixing the variable name with an underscore
    //    
    let _the_morpheus_ship = "Nebuchadnezzar";
}