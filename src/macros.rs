//-------------------------------------------------------------------------------------------
//---------------- ch19-06-macros ---------------------
//-------------------------------------------------------------------------------------------

use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

pub fn macros_example() {
    Pancakes::hello_macro();
}