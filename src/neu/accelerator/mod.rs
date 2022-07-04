use std::rc::Rc;

pub struct Accelerator<'a> {
    context: Rc<dyn Context + 'a>
}

pub(super) trait Context {
    
}