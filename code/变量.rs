// 变量
fn main() {
    // i8 i16 i32 i64 i128 isize
    // u8 u16 u32 u64 u128 usize
    let a_i32: i32 = 1024;

    // f32 f64
    let a_f32 = 1.4f32;

    // bool
    let a_bool = true;
    
    // char
    let a_char = 'a';

    // 自动推导
    let i = 1024;
    let b = true;

    // 不可变
    let unmut_num = 1024;
    unmut_num = 404; // 失败

    // 可变
    let mut mut_num = 1024;
    mut_num = 404; // 成功
    mut_num = true; // 失败


}
