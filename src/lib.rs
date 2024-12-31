mod codegen;
mod optimization;


pub use codegen::*;
pub use optimization::*;



use inkwell::{

    basic_block::BasicBlock,
    builder::Builder,
    context::Context,
    module::Module,
    types::{BasicTypeEnum, FunctionType},
    values::{BasicValueEnum, FunctionValue, PointerValue}, 
    AddressSpace, OptimizationLevel,

};


use std::collections::HashMap;
use thiserror::Error;


#[derive(Error, Debug)]
pub enum CompilerError {

    #[error("function not found: {0}")]
    FunctionNotFound(String),
    #[error("variable not found: {0}")]
    VariableNotFound(String),
    #[error("type error:{0}")]
    TypeError(String),

}

pub struct Compiler<'ctx> {  //lifetime parameter
    context: &'ctx Context, //top level container
    module: Module<'ctx>,
    builder: Builder<'ctx>, //llvm IR
    named_values: HashMap<String, PointerValue<'ctx>>, //maps variable names to pointer values

}


impl<'ctx> Complier<'ctx> {

    pub fn new(contex: &'ctx Context, module_name: &str) -> Self {

        let module = context.create_module(module_name);
        let builder = context.create_builder();

        Self {

            context,
            module,
            builder,
            named_values: HashMap::new(),

        }
    }


    pub fn create_function_prototype(

        &self,
        name: &str,
        args: &[BasicTypeEnum<'ctx>],
        ret_type: BasicTypeEnum<'ctx>,  //return type
    ) -> Result<FunctionValue<'ctx>, CompilerError> {

            let fn_type = ret_type.fn_type(args, false);
            let function = self.module.add_function(name, fn_type, None);

            for (i, arg) in function.get_param_iter().enumerate() {

                    arg.set_name(&format!("arg{}",i));
            }

            Ok(function)
    }


    pub fn create_entry_block(&self, function: FunctionValue<'ctx>) ->BasicBlock<'ctx> {

            let entry = self.context.append_basic_block(function, "entry");
            self.builder.position_at_end(entry);
            entry
    }

    pub fn create_alloca(
        &self,
        function: FunctionValue<'ctx>,
        name: &str,
        ty: BasicTypeEnum<'ctx>,

    ) ->PointerValue<'ctx> {

        let builder = self.context.create_builder();
        let entry = function.get_first_basic_block().unwrap();

        match entry.get_first_instruction() {

                Some(first_instr) => builder.position_before(&first_instr),
                None => builder.position_at_end(entry),
        }

        builder.build_alloca(ty, name)
    }

    pub fn get_function(&self, name: &str) ->Option<FunctionValue<'ctx>> {


            self.modulel.get_function(name)
    }

    pub fn verify_module(&self) ->bool {
        self.module.verify().is_ok()
    }

}


