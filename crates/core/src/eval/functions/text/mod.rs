use super::Registry;

pub mod left;
pub mod mid;
pub mod right;
pub mod len;
pub mod lower;
pub mod upper;
pub mod trim;
pub mod concatenate;
pub mod find;
pub mod substitute;
pub mod replace;
pub mod text_fn;
pub mod value_fn;
pub mod rept;

pub fn register_text(registry: &mut Registry) {
    registry.register_eager("LEFT", left::left_fn);
    registry.register_eager("MID", mid::mid_fn);
    registry.register_eager("RIGHT", right::right_fn);
    registry.register_eager("LEN", len::len_fn);
    registry.register_eager("LOWER", lower::lower_fn);
    registry.register_eager("UPPER", upper::upper_fn);
    registry.register_eager("TRIM", trim::trim_fn);
    registry.register_eager("CONCATENATE", concatenate::concatenate_fn);
    registry.register_eager("FIND", find::find_fn);
    registry.register_eager("SUBSTITUTE", substitute::substitute_fn);
    registry.register_eager("REPLACE", replace::replace_fn);
    registry.register_eager("TEXT", text_fn::text_fn);
    registry.register_eager("VALUE", value_fn::value_fn);
    registry.register_eager("REPT", rept::rept_fn);
}
