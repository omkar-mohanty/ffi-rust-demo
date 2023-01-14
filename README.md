# Working
The `add` function is defined in https://github.com/omkar-mohanty/ffi-rust-demo/blob/ab0354e54afeae2d1369aea0b5e3b78b1e494147/src/lib.rs#L2-L5.

The c++ code has a extern block which links to the rust function https://github.com/omkar-mohanty/ffi-rust-demo/blob/ab0354e54afeae2d1369aea0b5e3b78b1e494147/demo.cpp#L3.

The main function from c++ calls add on numbers `14` and `17`https://github.com/omkar-mohanty/ffi-rust-demo/blob/ab0354e54afeae2d1369aea0b5e3b78b1e494147/demo.cpp#L6-L8.

# Instructions
1. Build the cdylib
```
cargo build
```
2. compile c++ demo
``` 
g++ demo.cpp ./target/debug/libdemo.so
```
3. Run 
```
./a.out
```

