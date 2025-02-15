fn main() {
    // primary understanding
    // assign value as binary without sign
    let x:u8 = 0b1111;
    println!("dec {}", x);

    // other examples
    // assign negative value as binary
    input_negative_as_bin();
    // bit shift to nth
    shift_bit();

    // set nth bit is necessary for embedded hardware
    // suppose we have 8-bit register
    // we just have to set the 6th bit without modifying other bits
    // here default value of the register
    let mut reg:u8 = 0b10001111;
    println!("reg {:b}", reg);
    // 6th bit is 0; let's set it by the following function
    set_nth_bit(&mut reg, 6);
    println!("reg {:b}", reg);
    // let's clear the 0th bit
    clear_nth_bit(&mut reg, 0);
    println!("reg {:b}", reg)
}

fn set_nth_bit(x:&mut u8, n:u8){
    let msk:u8 = 1<<n;
    *x |= msk
}

fn clear_nth_bit(x:&mut u8, n:u8){
    let msk:u8 = !(1<<n);
    *x &= msk;
}

fn shift_bit(){
    println!("Bit Shifting Examples");
    let mut x = 0b0001;
    
    println!("bin {:b}", x);
    println!("shift <<2");
    x  = x<<2;
    println!("bin {:b}", x);

}




fn input_negative_as_bin(){
    // signed integer
    // input -2 as binary
    // MSB 0 means positive 1 means negative
    // in that way i8 range (-2^7 to -2^7)
    // Rust use 2's complement to represent negative numbers

    println!("Input Negative Number as Binary");
    
    let y = 0b11111110u8 as i8; 

    println!("bin {:b}",y);
    println!("dec {}", y);

}