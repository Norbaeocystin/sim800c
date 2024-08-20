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
    // custom header adding Authorization header
    // "AT+HTTPPARA=\"USERDATA]\",\"Authorization: Bearer [My Token]\"\r\n"
    info!("setting done");
    let post_data = "airSensors,sensor_id=TLM0201 temperature=73.97038159354763,humidity=35.23103248356096,co=0.48445310567793615 1630424257000000000";
    sim.send_command(&format!("AT+HTTPDATA={},{}\r", post_data.as_bytes().len(), 100000));
    sim.read(Some("DOWNLOAD"), 60_000, None);
    sim.port_opened.write_all(post_data.as_bytes());
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

GPRMC,182305.00,A,2517.65442,S,05734.48312,W,0.097,,200824,,,A*79
$GPVTG,,T,,M,0.097,N,0.180,K,A*24
$GPGGA,182305.00,2517.65442,S,05734.48312,W,1,03,3.21,63.0,M,13.8,M,,*6B
$GPGSA,A,2,29,11,20,,,,,,,,,,3.36,3.21,1.00*0D
$GPGSV,1,1,04,11,26,139,41,20,26,103,35,28,29,232,33,29,54,222,33*7F
$GPGLL,2517.65442,S,05734.48312,W,182305.00,A,A*62
$GPRMC,182306.00,A,2517.65439,S,05734.48293,W,0.121,,200824,,,A*72
$GPVTG,,T,,M,0.121,N,0.225,K,A*24
$GPGGA,182306.00,2517.65439,S,05734.48293,W,1,03,3.21,62.3,M,13.8,M,,*6E
$GPGSA,A,2,29,11,20,,,,,,,,,,3.36,3.21,1.00*0D
$GPGSV,1,1,04,11,26,139,42,20,26,103,35,28,29,232,33,29,54,222,33*7C
$GPGLL,2517.65439,S,05734.48293,W,182306.00,A,A*65
$GPRMC,182307.00,A,2517.65456,S,05734.48272,W,0.015,,200824,,,A*73
$GPVTG,,T,,M,0.015,N,0.027,K,A*22
$GPGGA,182307.00,2517.65456,S,05734.48272,W,1,03,3.21,61.9,M,13.8,M,,*60
$GPGSA,A,2,29,11,20,,,,,,,,,,3.37,3.21,1.00*0C
$GPGSV,1,1,04,11,26,139,42,20,26,103,35,28,29,232,33,29,54,222,33*7C
$GPGLL,2517.65456,S,05734.48272,W,182307.00,A,A*62
$GPRMC,182308.00,A,2517.65458,S,05734.48248,W,0.046,,200824,,,A*7D
$GPVTG,,T,,M,0.046,N,0.085,K,A*2C
$GPGGA,182308.00,2517.65458,S,05734.48248,W,1,03,3.21,61.7,M,13.8,M,,*66
$GPGSA,A,2,29,11,20,,,,,,,,,,3.37,3.21,1.00*0C
$GPGSV,2,1,05,11,26,139,42,20,26,103,35,27,,,22,28,29,232,34*44
$GPGSV,2,2,05,29,54,222,34*43
$GPGLL,2517.65458,S,05734.48248,W,182308.00,A,A*6A
$GPRMC,182309.00,A,2517.65473,S,05734.48225,W,0.077,,200824,,,A*7C
$GPVTG,,T,,M,0.077,N,0.143,K,A*25
$GPGGA,182309.00,2517.65473,S,05734.48225,W,1,03,3.21,61.7,M,13.8,M,,*65
$GPGSA,A,2,29,11,20,,,,,,,,,,3.37,3.21,1.00*0C
$GPGSV,2,1,05,11,26