use pyo3::prelude::*;
mod data;
pub use data::Cards;


#[pymodule]
fn organizer(m: &Bound<'_, PyModule>) -> PyResult<()> {

    m.add_class::<Cards>()?;
    Ok(())
}


