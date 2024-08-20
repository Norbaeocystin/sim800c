use std::io::Write;
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
    // this can be in one command - just something AT+XY=12;+STF=23;+LST=45
    sim.at_httppara_e("CID", HTTPPARA::I(1))?;
    // let msg = format!('AT+HTTPPARA="URL","www.sim.com"\r');
    sim.at_httppara_e("URL", HTTPPARA::S("webhook.site".to_string()));
    sim.at_httppara_e("REDIR", HTTPPARA::I(1))?;
    // UA - user_agent
    sim.at_httppara_e("CONTENT",HTTPPARA::S("application/json".to_string()))?;
    info!("setting done");
    let post_data = "airSensors,sensor_id=TLM0201 temperature=73.97038159354763,humidity=35.23103248356096,co=0.48445310567793615 1630424257000000000";
    sim.flush();
    sim.send_command(&format!("AT+HTTPDATA={},{}\r", post_data.as_bytes().len(), 10000));
    sim.read(Some("DOWNLOAD"), 60_000, None);
    info!("done download there");
    sim.port_opened.write_all(post_data.as_bytes());
    // sim.port_opened.write_all(b"\r\n");
    sim.read(Some("OK"), 60_000, None);
    sim.at_httpaction_e(HTTP_ACTION::POST)?;
    info!("action invoked");
    // // here needs to be wait for response
    sim.flush();
    // to known when the request is done or also check AT+HTTPSTATUS=?
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