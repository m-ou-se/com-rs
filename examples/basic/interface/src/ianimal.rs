use crate::Food;
use com::{com_interface, interfaces::iunknown::IUnknown, sys::HRESULT};

com_interface! {
    #[uuid("EFF8970E-C50F-45E0-9284-291CE5A6F771")]
    pub unsafe interface IAnimal: IUnknown {
        pub fn eat(&self, food: *const Food) -> HRESULT;
        pub fn happiness(&self) -> usize;
    }
}
