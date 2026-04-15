use super::super::{FunctionMeta, Registry};

pub mod fv;
pub mod irr;
pub mod npv;
pub mod nper;
pub mod pmt;
pub mod pv;
pub mod rate;

pub fn register_financial(registry: &mut Registry) {
    registry.register_eager("PMT",  pmt::pmt_fn,  FunctionMeta { category: "financial", signature: "PMT(rate, nper, pv)",       description: "Periodic payment for a loan" });
    registry.register_eager("NPV",  npv::npv_fn,  FunctionMeta { category: "financial", signature: "NPV(rate, value1,...)",     description: "Net present value" });
    registry.register_eager("IRR",  irr::irr_fn,  FunctionMeta { category: "financial", signature: "IRR(values, guess)",        description: "Internal rate of return" });
    registry.register_eager("PV",   pv::pv_fn,    FunctionMeta { category: "financial", signature: "PV(rate, nper, pmt)",       description: "Present value" });
    registry.register_eager("FV",   fv::fv_fn,    FunctionMeta { category: "financial", signature: "FV(rate, nper, pmt)",       description: "Future value" });
    registry.register_eager("RATE", rate::rate_fn,FunctionMeta { category: "financial", signature: "RATE(nper, pmt, pv)",       description: "Interest rate per period" });
    registry.register_eager("NPER", nper::nper_fn,FunctionMeta { category: "financial", signature: "NPER(rate, pmt, pv)",       description: "Number of payment periods" });
}
