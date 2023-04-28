use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::errors::ValResult;
use crate::input::Input;
use crate::recursion_guard::RecursionGuard;

use super::{BuildContext, BuildValidator, CombinedValidator, Extra, Validator};

#[derive(Debug, Clone)]
pub struct JonasValidator;

impl BuildValidator for JonasValidator {
    const EXPECTED_TYPE: &'static str = "jonas";

    fn build(
        _schema: &PyDict,
        _config: Option<&PyDict>,
        _build_context: &mut BuildContext<CombinedValidator>,
    ) -> PyResult<CombinedValidator> {
        Ok(Self {}.into())
    }
}

impl Validator for JonasValidator {
    fn validate<'s, 'data>(
        &'s self,
        py: Python<'data>,
        input: &'data impl Input<'data>,
        _extra: &Extra,
        _slots: &'data [CombinedValidator],
        _recursion_guard: &'s mut RecursionGuard,
    ) -> ValResult<'data, PyObject> {
        Ok(input.validate_jonas()?.into_py(py))
    }

    fn different_strict_behavior(
        &self,
        _build_context: Option<&BuildContext<CombinedValidator>>,
        ultra_strict: bool,
    ) -> bool {
        !ultra_strict
    }

    fn get_name(&self) -> &str {
        Self::EXPECTED_TYPE
    }

    fn complete(&mut self, _build_context: &BuildContext<CombinedValidator>) -> PyResult<()> {
        Ok(())
    }
}
