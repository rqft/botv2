use ucd::{model::bidi_class::BidiClass, serde_json, Ucd};
macro_rules! aw {
    ($e:expr) => {
        tokio_test::block_on($e)
    };
}

pub fn main() {
    let v = Ucd::new();

    // dbg!(serde_json::to_value(BidiClass::ArabicNumber));

    let x = aw!(v.hex(61));
    // dbg!(x);
}
