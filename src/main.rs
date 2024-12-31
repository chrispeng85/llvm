use inkwell::context::Context;
use rust_llvm::{Compiler, CodeGen, Optimizer};

mod lib;
mod codegen;
mod optimization;

fn main() {

    env_logger::init();
    
    let context = Context::create();
    let compiler = Compiler::new(&context, "example_module");

    let i32_type = context.i32_type().into();
    let args = vec![i32_type, i32_type];

    match compiler.create_function_prototype("add", &args, i32_type) {

        OK(function) => {

            let entry = compiler.create_entry_block(function);
            println!("successfully created function 'add'");
        }

        Err(e) => eprintln!("error creating function: {}", e),

    }

    if compiler.verify_module() {

            println!("module verification successful");
    }

    else {

        eprintln!("module verification failed");
    }
 
}