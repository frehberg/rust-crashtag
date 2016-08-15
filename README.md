# rust-crashtag

'crashtag' provides tooling for crashtags for Rust for post-mortem analysis.

Crashtags are placed in context of main() function or on stack of any other thread. These tags will placed on stack and in case the application is crashing, these tags are embedded into the core dump file.

Post-mortem these tags can be extracted from core-dump file, for example release informatiop or application config.

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
