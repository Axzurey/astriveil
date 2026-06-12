use cached::proc_macro::cached;

#[cached]
pub fn local_xyz_to_index(x: u32, y: u32, z: u32) -> u32 {
    ((y * 16 * 16) + (z * 16) + x) as u32
}
pub fn index_to_local_xyz(index: u32) -> (u32, u32, u32) {
    let x = index % 16;
    let z = (index / 16) % 16;
    let y = (index / 256) % 16;
    (x, y, z)
}

#[cached]   
pub fn xz_to_index(x: i32, z: i32) -> u32 {
    let x0 = if x >= 0 {2 * x} else {-2 * x - 1}; //converting integers to natural numbers
    let z0 = if z >= 0 {2 * z} else {-2 * z - 1};

    (0.5 * (x0 + z0) as f32 * (x0 + z0 + 1) as f32 + z0 as f32) as u32 //cantor pairing https://math.stackexchange.com/questions/3003672/convert-infinite-2d-plane-integer-coords-to-1d-number
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        use colored::Colorize;
        println!("{} {}", "[WARNING]:".yellow(), format!($($arg)*).yellow())
    }
}
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        use colored::Colorize;
        println!("{} {}", "[ERROR]:".red(), format!($($arg)*).red())
    }
}