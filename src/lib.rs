use eframe::{App, AppCreator, CreationContext, NativeOptions, Result, run_native};

pub trait Creatable {
    fn new(creator: &CreationContext) -> Self;
}

pub fn run<A: App + Creatable>(name: &str, options: NativeOptions) -> Result {
    let creator: AppCreator = Box::new(|creator| Ok(Box::new(A::new(creator))));

    run_native(name, options, creator)
}
