use super::super::Registry;

pub mod if_fn;
pub mod and;
pub mod or;
pub mod not;
pub mod iferror;
pub mod ifs;
pub mod switch;
pub mod is_checks;
pub mod constants;
pub mod xor;
pub mod info;

pub fn register_logical(registry: &mut Registry) {
    registry.register_lazy("IF", if_fn::if_fn);
    registry.register_lazy("AND", and::and_fn);
    registry.register_lazy("OR", or::or_fn);
    registry.register_eager("NOT", not::not_fn);
    registry.register_lazy("IFERROR", iferror::iferror_fn);
    registry.register_lazy("IFNA", iferror::ifna_fn);
    registry.register_lazy("IFS", ifs::ifs_fn);
    registry.register_lazy("SWITCH", switch::switch_fn);
    registry.register_lazy("ISNUMBER",  is_checks::isnumber_lazy_fn);
    registry.register_lazy("ISTEXT",    is_checks::istext_lazy_fn);
    registry.register_lazy("ISERROR",   is_checks::iserror_lazy_fn);
    registry.register_lazy("ISBLANK",   is_checks::isblank_lazy_fn);
    registry.register_lazy("ISNA",      is_checks::isna_lazy_fn);
    registry.register_lazy("ISERR",     is_checks::iserr_fn);
    registry.register_lazy("ISLOGICAL", is_checks::islogical_fn);
    registry.register_lazy("ISNONTEXT", is_checks::isnontext_fn);
    registry.register_eager("NA",    constants::na_fn);
    registry.register_eager("TRUE",  constants::true_fn);
    registry.register_eager("FALSE", constants::false_fn);
    registry.register_lazy("XOR", xor::xor_fn);
    registry.register_lazy("ERROR.TYPE", info::error_type_fn);
    registry.register_lazy("N",          info::n_fn);
    registry.register_lazy("TYPE",       info::type_fn);
}
