//making module of lib.rs file
mod lib;
//using use keyward to access the lib.rs file
use lib::Hotel::floors;


fn main() {
    crate::lib::Hotel::floors::room();// absolute path
    //Use keyward
    floors::room();
}
