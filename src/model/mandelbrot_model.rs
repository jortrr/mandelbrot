use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref SINGLETON_INSTANCE: Mutex<MandelbrotModel> = Mutex::new(MandelbrotModel::new());
}

struct MandelbrotModel {}

impl MandelbrotModel {
    fn new() -> MandelbrotModel {
        MandelbrotModel {}
    }
}
