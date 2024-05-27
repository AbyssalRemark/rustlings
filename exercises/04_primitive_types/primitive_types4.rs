// primitive_types4.rs
//
// Get a slice out of Array a where the ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand
// for a hint.

// I AM DONE

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1..4]; // I had to borrow this to compile.. *sigh* ok. 
    //Like I get why, we if we copied we would need to know at compile time but
    //if were just borrowing it, were not making memory. So it need not be static

    assert_eq!([2, 3, 4], nice_slice)
}
