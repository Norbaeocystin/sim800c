use anyhow::{anyhow, Result};
use log::{info, LevelFilter};
use sim800c::sim800c::Sim800C;

fn main() -> Result<()>{
    // other option - to get at+ceng=2 and approximate from the data
    // to stop at+ceng=1
    env_logger::builder().filter_level(LevelFilter::Info).init();
    let mut sim = Sim800C::new(
        "/dev/ttyUSB0".to_string(),
        115_200,
        "internet".to_string(),
        10_000,
    );
    sim.at()?;
    sim.at_cmee_e(2)?;
    info!("at done");
    // AT+CGATT =1
    let query = sim.query_gprs_context();
    if query.contains("\"0.0.0.0\"") {
        // simple get request
        sim.set_gprs()?;
        info!("set gprs");
        sim.set_apn()?;
        info!("set apn");
        sim.open_gprs_context()?;
        info!("opened gprs context");
        let query = sim.query_gprs_context();
        info!("gprs opened {}", query);
    } else {
        info!("gprs context already there {}", query);
    };
    sim.send_command("AT+CLBS=1,1\r");
    sim.read(Some("OK"),10_000, Some("+CLBS:") );
    // <ret_code>[,<latitude>,<longitude>,<acc>,<date>,<time>]
    sim.close_gprs_context()?;
    info!("closed gprs context");
    Ok(())
}
