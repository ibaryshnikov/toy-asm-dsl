#[macro_use]
mod dsl;

fn main() {
    asm! {
        mov ebx 5;
        mov edx 3;
        mul ebx edx;
        print ebx;
        add ebx edx;
        print ebx
    };
}
