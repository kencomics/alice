use inkwell::{builder::Builder, context::Context, module::Module as LLVMModule, AddressSpace};

use crate::ast::{Alias, Module, Stmt};

struct CodeGen<'ctx> {
    context: &'ctx Context,
    llvm_module: LLVMModule<'ctx>,
    builder: Builder<'ctx>,
    module: Module,
}

impl<'ctx> CodeGen<'ctx> {
    fn create(context: &'ctx Context, module: Module) -> CodeGen<'ctx> {
        CodeGen::<'ctx> {
            context,
            llvm_module: context.create_module(&module.name.as_str()),
            builder: context.create_builder(),
            module,
        }
    }

    fn generate_module(&self) {
        for stmt in self.module.stmts.as_slice() {
            self.generate_stmt(stmt);
        }
    }

    fn generate_stmt(&self, stmt: &Stmt) {
        match stmt {
            Stmt::Alias(alias) => self.generate_alias(alias),
        }
    }

    fn generate_alias(&self, alias: Alias) {
        let address_space = AddressSpace::from(1u16);
        let int_type = self.context.i32_type();
        let global =            self.llvm_module
                .add_global(int_type, Some(address_space), alias.name.as_str());
    }
}
