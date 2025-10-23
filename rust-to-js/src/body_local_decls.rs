use crate::*;

impl<'tcx> MyVisitor<'_, 'tcx> {
    pub fn on_body_local_decls(&mut self, body: &Body<'tcx>) {
        for (i, local_decl) in body.local_decls.iter().enumerate() {
            self.out(format!("let _{i} = ",));

            self.out(match local_decl.ty.ref_mutability() {
                None => "new NoRefVar(",
                Some(mutability) => match mutability {
                    Mutability::Not => "new RefVar(",
                    Mutability::Mut => "new MutRefVar(",
                },
            });

            self.out(format!(
                "sizeof({}));\n",
                if let Some(size) = self.sizeof(&local_decl.ty) {
                    size.to_string()
                } else {
                    local_decl.ty.to_string()
                }
            ));
        }
    }
}
