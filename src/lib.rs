#[macro_use]
extern crate ruru;

use std::error::Error;
/*
use ruru::VM;
use ruru::Hash;
use ruru::Fixnum;
use ruru::Class;
use ruru::AnyObject;
*/
// Merge above ruru Structs
use ruru::{VM, Hash, Fixnum, Class, Object};

// ruru Changes and deprecated
// https://github.com/d-unseductable/ruru/wiki/Upgrading-from-0.7-to-0.8
// use ruru::types::{Argc, Value};
// use ruru::traits::Object; .

class!(RustyCalculator);

methods!(
    RustyCalculator,
    itself,

    fn pow_3(num: Fixnum) -> Hash {
        let mut hash = Hash::new();

        // `num` is not a valid `Fixnum`, raise an exception
        if let Err(ref error) = num {
            VM::raise(error.to_exception(), error.description());
        }

        // We can safely unwrap here, because an exception was raised if `num` is `Err`
        for i in 1..num.unwrap().to_i64() + 1 {
            hash.store(Fixnum::new(i), Fixnum::new(i.pow(3)));
        }

        hash
    }
);

#[no_mangle]
pub extern fn initialize_my_app() {
    Class::new("RustyCalculator", None).define(|itself| {
        itself.def("pow_3", pow_3);
    });
}
