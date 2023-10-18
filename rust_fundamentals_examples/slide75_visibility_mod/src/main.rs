mod my_module {
    pub struct PublicStruct { } 
    struct PrivateStruct { }
    pub fn public_function() { }
    fn private_function() { }
    pub(crate) fn crate_function() { }
    pub(in crate::my_module::nested) fn restricted_function() { }
}
 
fn main() {
    let public_struct = my_module::PublicStruct {};
    my_module::public_function();
    // let private_struct = my_module::PrivateStruct {}; // Error: not accessible here
    // my_module::private_function(); 									// Error: not accessible here
}