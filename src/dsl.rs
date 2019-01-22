#[macro_export]
macro_rules! asm {
    (print $name:ident) => {
        println!("{} is {}", stringify!($name), $name);
    };
    (mov $name:ident $value:expr) => {
        let $name = $value;
    };
    (add $to:ident $from:expr) => {
        let mut $to = $to;
        $to += $from;
    };
    (sub $to:ident $from:expr) => {
        let mut $to = $to;
        $to -= $from;
    };
    (mul $to:ident $from:expr) => {
        let mut $to = $to;
        $to *= $from;
    };
    (div $to:ident $from:expr) => {
        let mut $to = $to;
        $to /= $from;
    };
    ($pattern:ident $name:ident $($value:expr)?) => {
        panic!(format!("pattern for '{}' not implemented", stringify!($pattern)));
    };
    ($($pattern:ident $name:ident $($value:expr)?);*) => {
        $(asm!($pattern $name $($value)?);)*
    };
}
