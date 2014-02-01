# LLVM With Rust

This repository is an example of linking, and using LLVM from with the Rust language. The only reference examples are within the Rust compiler's source code, which can be very cryptic given the amount of extra context there is.

Linking LLVM with Rust is **dead simple**. I really mean it. You simply need to link against the `rustc` (dynamic) library, which contains the Rust compiler, and LLVM Rust bindings. This library already has the LLVM library staticly linked against it.

# Usage

Fetch this repo:

```
git clone x
```

And build and run it:

```
make run
```

This will create a binary within the `./bin/` directory called `rustllvm`.

# Overview

By default, we simply create an empty Context, Builder and Module. We finally print the contents of the module (`module->dump()` in C++ terms) to stderr.

You can start emitting LLVM IR from within the context and module, afterwhich you should see more than just:

```
; ModuleID = 'mod1'
```

As an output.

# License

MIT