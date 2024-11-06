pub mod selector;
pub mod renderer;
mod grid_shader;

pub fn u32_to_u8(arr: &[u32]) -> &[u8] {
    let len = 4 * arr.len();
    let ptr = arr.as_ptr() as *const u8;
    unsafe {
        std::slice::from_raw_parts(ptr, len)
    }
}