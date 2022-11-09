fn main() {
    /* ========== SCALAR TYPES ========== */

        // Integer Types -> -(2**bit-1) to (2**bit-1)-1;

            let x: i8 = 1;
            let x: u8 = 1;
            let x: i16 = 1;
            let x: u16 = 1;
            let x: i32 = 1;
            let x: u32 = 1;
            let x: i64 = 1;
            let x: u64 = 1;
            let x: isize = 1;
            let x: usize = 1;

        // Floating-Point Types -> all are signed

            let y: f32 = 2.0;
            let y: f64 = 2.0;

        // Boolean Type

            let boo: bool = true || false;

        // Character Type

            let c: char = 'L'; // 4 bytes

    /* ========== COMPOUND TYPES ========== */

        // Tuple Type

            let tup: (i32, f64, u8) = (500, 6.4, 1);
            let (x, y, z) = tup; // x = 500 | y = 6.4 | z = 1
        
        // Array Type

            let arr = [1, 2, 3];

            let [x, y, z] = arr; // x = 1 | y = 2 | z = 3            
}
