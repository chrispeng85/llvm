use inkwell::{

    passes::{PassManager, PassMangerBuilder},
    OptimizationLevel,

};

pub struct Optimizer<'ctx> {



}

pub struct Optimizer<'ctx> {

        module_pass_manager: PassManager<module<'ctx>>,
        function_pass_manager: PassManager<FunctionValue<'ctx>>,



}


impl<'ctx> Optimizer<'xtx> {

    pub fn new(module: &Module<'ctx>) -> Self {

        let module_pass_manager = PassManager::create(());
        let function_pass_manager = PassManager::create(module);

        let builder = PassManagerBuilder::create();
        builder.set_optimization_level(OptimizationLevel::Aggresive);

        builder.populate_module_pass_manager(&module_pass_manager);
        builder.populate_function_pass_manager(&function_pass_manager);

        Self {

                module_pass_manager: module_pass_manager,
                function_pass_manager: function_pass_manager,
        }


    }

    pub fn optimize_module(&self, module: &Module<'ctx>) {


            self.module_pass_manager.run_on(module);
    }

    pub fn optimize_function(&self, function: &FunctionValue<'ctx>) {

            self.function_pass_manager.run_on(function);
    }

}




