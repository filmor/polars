use std::borrow::Cow;

use polars_core::prelude::*;

use crate::physical_plan::state::ExecutionState;
use crate::prelude::*;

pub(crate) struct ExplodeExec {
    pub(crate) input: Box<dyn Executor>,
    pub(crate) columns: Vec<String>,
}

impl Executor for ExplodeExec {
    fn execute(&mut self, state: &mut ExecutionState) -> PolarsResult<DataFrame> {
        #[cfg(debug_assertions)]
        {
            if state.verbose() {
                println!("run ExplodeExec")
            }
        }
        let df = self.input.execute(state)?;
        state.record(|| df.explode(&self.columns), Cow::Borrowed("explode()"))
    }
}
