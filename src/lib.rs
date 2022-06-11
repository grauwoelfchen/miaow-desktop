use gdnative::prelude::*;
use gdnative::core_types::GodotString;

#[derive(NativeClass)]
#[inherit(Label)]
pub struct Miaow;

#[methods]
impl Miaow {
    fn new(_owner: &Label) -> Self {
        Self
    }

    #[export]
    fn _ready(&self, owner: &Label) {
        owner.set_text(GodotString::from("Miaow!"));
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<Miaow>();
}

godot_init!(init);
