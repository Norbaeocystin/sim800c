use anyhow::{anyhow, Result};
use log::{info, LevelFilter};
use sim800c::sim800c::Sim800C;

fn main() {
    // other option - to get at+ceng=2 and approximate from the data
    // to stop at+ceng=1
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
    sim.close_gprs_context()?;
    info!("closed gprs context");
    Ok(())
}

// OK
// AT+SAPBR =3,1,”CONTYPE”,”GPRS”
// ERROR
// AT+SAPBR=1,1
// OK
// AT+SAPBR=2,1
//     +SAPBR: 1,1,"10.97.199.43"
//
// OK
// AT+CIPGSMLOC=1,1
//     +CIPGSMLOC: 0,0.000000,0.000000,2024/08/21,00:33:02
//
// OK
// AT+CIPGSMLOC=2,1
//     +CIPGSMLOC: 0,2024/08/21,00:33:19
//
// OK
// at+clbs=1,1
//     +CLBS: 0,-57.573456,-25.289178,550
//
// OK
// at+sapbr=0,1
// OK
