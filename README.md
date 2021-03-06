# rust-crashtag

## DEPRECATION!!

THIS CRATE HAS BEEN RENAMED TO
https://crates.io/crates/releasetag

The crate crashtag will be removed soon.

# Introduction
The crate 'crashtag' provides tooling for for post-mortem analysis for Rust .

Crashtags are placed in context of main() function or on stack of any other thread. These tags will placed on stack and in case the application is crashing, these tags are embedded into the core dump file.

Post-mortem these tags can be extracted from core-dump file, for example release information or application config.

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

Limitations:
Length of tag string is limited to 62 characters right now. Max length is defined by constant. In future this contstant should be replaced by actual length of provided tag-string.
