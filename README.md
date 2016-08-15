# rust-crashtag
Providing tooling for crashtags for Rust.

Crashtags are placed on stack of main() function. These tags will be written to core-dump file in case of any crash. 

These tags can be extraced from core-dump file, for example release informatiop or application config.

Example: file main.rs
```
#![feature(asm)] 
#[macro_use(crashtag)]
extern crate crashtag;

fn main() {
    crashtag!("BUILD_TAG=MAIN_2016-wk16-05");
    crashtag!("BUILD_HOST=host1");

}
```
In case the application coredumps to file 'core' the following comamnd can be used to extract the tags from core-file:
```
cat core | strings | grep BUILD_
```
