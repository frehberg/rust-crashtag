#![feature(asm)] 

#[macro_export]
macro_rules! crashtag {
    ($tag:expr) => {{
       // println!("{}", $s);
        let mut stackmem : &mut[u8; 64] = &mut[0; 64];
        stackmem[0] = '\0' as u8; // delimit from preceding string
        
        // copy tag into stack mem
        for (i,c) in $tag.chars().enumerate() {
            stackmem[i+1] = c as u8;
        }
        // nop to force linker to preserve the variable on stack
        unsafe { asm!("" : : "r"(&stackmem)) }
    }};
}


#[cfg(test)]
mod tests {
    #[test]
    fn valid_macro() {
        crashtag!("TAG1=123");
        crashtag!("TAG2=ABC");
    }
}
