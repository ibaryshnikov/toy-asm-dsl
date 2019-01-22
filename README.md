### A toy asm-like dsl
It's an example of a dsl built with rust declarative macro

The set of instructions only contains `mov`, `add`, `sub`, `mul`, `div`

#### Usage example
```
asm! {
    mov ebx 1;
    mov edx 2;
    add ebx edx;
    print ebx // should print 3
}
```
There is no `print` instruction in asm, but it's useful in examples, so I added it

#### Building
Edit `main.rs` and than use `cargo run` to run it

#### Goalds
There is no goal to create a correct asm implementation.

Instead I made a simplified dsl to show how asm code may look,
without the need of setting up the real environment or going too deep into the
low level details
