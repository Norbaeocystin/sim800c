use anyhow::{anyhow, Result};
use log::{info, LevelFilter};
use sim800c::sim800c::{Sim800C, HTTPPARA, HTTP_ACTION};

fn main() -> Result<()> {
    env_logger::builder().filter_level(LevelFilter::Info).init();
    let mut sim = Sim800C::new(
        "/dev/tty.usbserial-110".to_string(),
        115_200,
        "internet".to_string(),
        60_000,
    );
    sim.at()?;
    sim.at_cmee_e(2)?;
    info!("at done");
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
    }
    sim.at_httpinit()?;
    info!("http init done");
    // sim.https_on()?;
    // info!("https on");
    sim.at_httppara_e("CID", HTTPPARA::I(1))?;
    // let msg = format!('AT+HTTPPARA="URL","www.sim.com"\r');
    sim.at_httppara_e("URL", HTTPPARA::S("google.com".to_string()));
    sim.at_httppara_e("REDIR", HTTPPARA::I(1))?;
    info!("setting done");
    sim.at_httpaction_e(HTTP_ACTION::GET)?;
    info!("action invoked");
    // // here needs to be wait for response
    sim.flush();
    let out = sim.read(Some("\n"), 60_000, Some("+HTTPACTION"));
    info!("wait {}", out);
    let out = sim.at_httpread();
    info!("{}", out);
    sim.at_httpterm()?;
    info!("closed connection");
    // sim.close_gprs_context()?;
    info!("closed gprs context");
    Ok(())
}
