use crate::*;

impl<'tcx> MyVisitor<'_, 'tcx> {
    pub fn on_body_local_decls(&mut self, body: &Body<'tcx>) {
        for (i, local_decl) in body.local_decls.iter().enumerate() {
            println!("local_decl.ty: {:?}", local_decl.ty);
            let size = self.sizeof(&local_decl.ty).unwrap();
            self.outln(format!("const _{i} = stackAlloc({size});"));
            if i > 0 && i <= body.arg_count {
                self.outln(format!("assign(_{i}, arg{});", i - 1));
            }
        }
    }
}
