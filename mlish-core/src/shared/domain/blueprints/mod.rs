use std::fmt::Display;

crate::inline_mod!(rpc);

trait Injectable {
    fn init(&self);
}
