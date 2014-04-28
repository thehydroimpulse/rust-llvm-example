extern crate rustc = "rustc#0.11-pre";

use rustc::lib::llvm::llvm;

fn main() {
    unsafe {
        // Create our first global context.
        let llvm_context = llvm::LLVMContextCreate();

        // Create our module `module1` and attach our context.
        let llvm_module = "mod1".with_c_str(|buf| {
            llvm::LLVMModuleCreateWithNameInContext(buf, llvm_context)
        });

        let builder = llvm::LLVMCreateBuilderInContext(llvm_context);
        llvm::LLVMDumpModule(llvm_module);
    }
}
