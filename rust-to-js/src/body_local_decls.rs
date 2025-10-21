use crate::*;

impl<'tcx> MyVisitor<'tcx> {
    pub fn on_body_local_decls(&mut self, body: &Body<'tcx>) {
        let mut output = String::new();

        for (i, local_decl) in body.local_decls.iter().enumerate() {
            output += &format!("let _{i} = ");

            let size = self.sizeof(&local_decl.ty);
            output += &match local_decl.ty.ref_mutability() {
                None => format!("new NoRefVar(memory.stackAlloc({size}), {size});"),
                Some(mutability) => match mutability {
                    Mutability::Not => format!("new RefVar(memory.stackAlloc({size}), {size});"),
                    Mutability::Mut => format!("new MutRefVar(memory.stackAlloc({size}), {size});"),
                },
            };
        }

        self.output += &output;
    }
}
