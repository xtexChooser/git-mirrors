use anyhow::Result;

mod antianaly;
mod antidbg;
mod c;

pub fn check_environment() -> Result<()> {
    antianaly::detect_analysis_tools()?;
    antidbg::detect_debuggers()?;
    Ok(())
}
