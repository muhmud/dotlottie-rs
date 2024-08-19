#[repr(C)]
pub enum MyEnum {
    VariantA = 0,
    VariantB = 1,
    VariantC = 2,
}

#[no_mangle]
pub extern "C" fn addd(a: i32, b: i32) -> MyEnum {
    MyEnum::VariantA
}
