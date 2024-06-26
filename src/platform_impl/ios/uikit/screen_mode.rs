use objc2::{extern_class, extern_methods, mutability, ClassType};
use objc2_foundation::{CGSize, NSObject};

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub(crate) struct UIScreenMode;

    unsafe impl ClassType for UIScreenMode {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
    }
);

extern_methods!(
    unsafe impl UIScreenMode {
        #[method(size)]
        pub fn size(&self) -> CGSize;
    }
);
