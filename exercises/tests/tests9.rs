// tests9.rs
//
// Rust is highly capable of sharing FFI interfaces with C/C++ and other statically compiled
// languages, and it can even link within the code itself! It makes it through the extern
// block, just like the code below.
//
// The short string after the `extern` keyword indicates which ABI the externally imported
// function would follow. In this exercise, "Rust" is used, while other variants exists like
// "C" for standard C ABI, "stdcall" for the Windows ABI.
//
// The externally imported functions are declared in the extern blocks, with a semicolon to
// mark the end of signature instead of curly braces. Some attributes can be applied to those
// function declarations to modify the linking behavior, such as #[link_name = ".."] to
// modify the actual symbol names.
//
// If you want to export your symbol to the linking environment, the `extern` keyword can
// also be marked before a function definition with the same ABI string note. The default ABI
// for Rust functions is literally "Rust", so if you want to link against pure Rust functions,
// the whole extern term can be omitted.
//
// Rust mangles symbols by default, just like C++ does. To suppress this behavior and make
// those functions addressable by name, the attribute #[no_mangle] can be applied.
//
// In this exercise, your task is to make the testcase able to call the `my_demo_function` in
// module Foo. the `my_demo_function_alias` is an alias for `my_demo_function`, so the two
// line of code in the testcase should call the same function.
//



    extern "Rust" {
        #[link_name = "my_demo_function"]
        fn my_demo_function(a: u32) -> u32;
        #[link_name = "my_demo_function"]
        fn my_demo_function_alias(a: u32) -> u32;
    }
    


mod Foo {
    // No `extern` equals `extern "Rust"`.
    #[no_mangle]
    fn my_demo_function(a: u32) -> u32 {
        a
    }
}
#[no_mangle]

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        // The externally imported functions are UNSAFE by default
        // because of untrusted source of other languages. You may
        // wrap them in safe Rust APIs to ease the burden of callers.
        //
        // SAFETY: We know those functions are aliases of a safe
        // Rust function.
        unsafe {
            my_demo_function(123);
            my_demo_function_alias(456);
        }
    }
}
// //`#[no_mangle]` 和 `#[link_name = "my_demo_function"]` 是Rust语言中的两个不同的属性，它们用于控制函数名称的符号重整（name mangling）和在链接（linking）过程中的符号名称。
// ### `#[no_mangle]`
// Rust默认对函数名称进行符号重整，这意味着编译器会根据函数的名称、参数类型和返回类型生成一个唯一的符号名称。这对于避免符号冲突很有用，但它也意味着Rust函数的名称在编译后的二进制文件中与源代码中的名称不同。
// `#[no_mangle]`属性用于告诉Rust编译器不要对带有这个属性的函数进行符号重整。当使用`#[no_mangle]`时，函数在编译后的二进制文件中将会具有源代码中指定的确切名称。这对于创建C语言兼容的函数接口（FFI）非常有用，因为C语言编译器不进行符号重整，所以Rust代码必须使用未重整的名称才能与C代码链接。
// 例如：
// ```rust
// #[no_mangle]
// pub fn my_c_compatible_function() {
//     // ...
// }
// ```
// 在上面的例子中，`my_c_compatible_function`在编译后的二进制中会有确切的名称`my_c_compatible_function`，这使得C语言能够通过这个名称调用这个函数。
// ### `#[link_name = "my_demo_function"]`
// `#[link_name = "my_demo_function"]`属性用于显式地指定函数在链接过程中的符号名称。当你使用这个属性时，你是在告诉编译器和链接器使用指定的符号名称而不是默认生成的名称。这在以下几种情况下非常有用：
// 1. 当你想要从Rust调用一个外部库中的函数，而这个函数的Rust名称与外部库中的符号名称不匹配时。
// 2. 当你想要为一个函数提供不同的符号名称，而不是它在Rust源代码中的名称时。
// 3. 当你需要控制或修复由于符号重整导致的链接问题。
// 在提供的代码示例中，`#[link_name = "my_demo_function"]`被用于两个函数声明：
// ```rust
// extern "Rust" {
//     #[link_name = "my_demo_function"]
//     fn my_demo_function(a: u32) -> u32;
//     #[link_name = "my_demo_function"]
//     fn my_demo_function_alias(a: u32) -> u32;
// }
// ```
// 在这里，`my_demo_function_alias`的`#[link_name]`属性表明，即使它有一个不同的Rust名称，它也应该链接到与`my_demo_function`相同的符号。这样，调用`my_demo_function_alias`实际上会调用`my_demo_function`的实现。
// 总结一下，`#[no_mangle]`用于防止Rust编译器对函数名称进行符号重整，而`#[link_name = "my_demo_function"]`用于指定函数在链接过程中的确切符号名称。在FFI场景中，这两个属性经常一起使用，以确保Rust函数可以被外部代码正确地调用。
