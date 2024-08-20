use anyhow::Result;
use sim800c::sim800c::Sim800C;

fn main() -> Result<()> {
    let mut sim = Sim800C::new(
        "/dev/tty.usbserial-10".to_string(),
        115_200,
        "internet".to_string(),
        10_000,
    );
    sim.at()?;
    let result = sim.at_cpin_q()?;
    println!("{:?}", result);
    sim.at_cmgf_e(1)?;
    // sim.at_cpms_e_sm();
    let out = sim.at_cmgl_e_all();
    println!("{}", out);
    sim.at_cmgs_e("+959872599", "Que ondaaaa?");
    Ok(())
}
