use inkwell::{

    values::{BasicValueEnum, FunctionValue},
    types::BasicTypeEnum,

};

use crate::CompilerError;

pub trait CodeGen<'ctx> {

    fn codegen(&self, compiler: &Compiler<'ctx>) -> Result<BasicValueEnum<'ctx>, CompilerError>;


}

pub struct BinaryExpr<'ctx> {

        pub op: String,
        pub lhs: Box<dyn CodeGen<'ctx>>,
        pub rhs: Box<dyn COdeGen<'ctx>>,
}

impl<'ctx> CodeGen <'ctx> for BinaryExpr<'ctx> {

    fn codegen(&self, compiler: &Compiler<'ctx>) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        let lhs = self.lhs.codegen(compiler)?;
        let rhs = self.rhs.codegen(compiler)?;

        if let (BasicValueEnum::IntValue(lhs), BasicValueEnum::IntValue(rhs)) = (lhs, rhs) {

            let result = match self.op.as_str() {

                "+" => compiler.builder.build_int_add(lhs, rhs, "addtmp"),
                "-" => compiler.builder.build_int_sub(lhs, rhs, "subtmp"),
                "*" => compiler.builder.build_int_mul(lhs, rhs, "multmp"),
                "/" => compiler.builder.build_int_signed_div(lhs, rhs, "divtmp"),
                _ => return Err(CompilerError::TypeError(format!("unknown operator: {}", self.op))),

            };

            Ok(BasicValueEnum::IntValue(result))

        }

        else {
            Err(CompilerError::TypeError("invalid operand types for binary expression").to_string())


        }
    }
    


}


