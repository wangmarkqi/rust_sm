#[link(name = "sm", kind = "static")]
extern "C" {
    fn Hi();
}

pub fn hi() -> anyhow::Result<()> {
    unsafe {
        Hi();
    }
    Ok(())
}
