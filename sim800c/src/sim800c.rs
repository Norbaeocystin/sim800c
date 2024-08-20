use anyhow::{anyhow, Result};
use log::{debug, error, info, warn};
use serialport::{Error, ErrorKind, SerialPort};
use std::fmt::format;
use std::io;
use std::io::Write;
use std::string::ToString;
use std::time::{Duration, SystemTime};

pub struct Sim800C {
    pub apn: String,
    pub baudrate: u32,
    pub port_opened: Box<dyn SerialPort>,
    pub timeout_ms: u128,
}

impl Sim800C {
    pub fn new(port: String, baudrate: u32, apn: String, timeout_ms: u64) -> Self {
        let mut port_opened = serialport::new(port, baudrate)
            .timeout(Duration::from_millis(timeout_ms))
            .open()
            .expect("Failed to open port");
        port_opened.write_data_terminal_ready(true);
        Self {
            apn,
            baudrate,
            port_opened: port_opened,
            timeout_ms: timeout_ms as u128,
        }
    }

    // example of command: AT\r
    pub fn send_command(&mut self, cmd: &str) {
        self.port_opened.write_all(cmd.as_bytes()).unwrap()
    }

    pub fn flush(&mut self) {
        self.port_opened.flush().unwrap();
    }

    pub fn read(
        &mut self,
        suffix_to_terminate: Option<&str>,
        timeout_ms: u128,
        contains: Option<&str>,
    ) -> String {
        let mut buffer: [u8; 1] = [0; 1];
        let mut output = String::new();
        let start = SystemTime::now();
        loop {
            match self.port_opened.read(&mut buffer) {
                Ok(bytes) => {
                    if bytes == 1 {
                        let char = std::str::from_utf8(&buffer);
                        match char {
                            Ok(char) => {
                                debug!("char {} {}", char, &buffer[0]);
                                output += char;
                            }
                            Err(_) => {
                                warn!("utf8 can't parse char")
                            }
                        }
                        if let Some(suffix) = suffix_to_terminate {
                            if output.ends_with(suffix) && contains.is_none() {
                                break;
                            }
                            if let Some(contains) = contains {
                                if output.ends_with(suffix) && output.contains(contains) {
                                    break;
                                }
                            }
                        }
                    }
                }
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                Err(e) => error!("{:?}", e),
            }
            let elapsed = start.elapsed().unwrap().as_millis();
            // println!("{}", elapsed);
            if elapsed > timeout_ms {
                break;
            }
        }
        output
    }
}

#[derive(Default, Debug, Eq, PartialEq)]
pub enum CPIN {
    #[default]
    READY,
    ERROR(String),
    UNKNOWN(String),
}

#[derive(Default, Debug, Eq, PartialEq)]
pub enum HTTP_ACTION {
    #[default]
    GET = 0,
    POST = 1,
    HEAD = 2,
}

#[derive(Default, Debug, Eq, PartialEq)]
pub enum HTTPS {
    #[default]
    OFF,
    ON,
}

pub enum HTTPPARA {
    S(String),
    I(u8),
}

// at commands
impl Sim800C {
    /// at
    pub fn at(&mut self) -> Result<()> {
        self.flush();
        self.send_command("AT\r");
        let out = self.read(Some("OK"), self.timeout_ms, None);
        if !out.ends_with("OK") {
            Err(anyhow!("{}", out))
        } else {
            Ok(())
        }
    }

    /// set logs cmee - 0,1,2 - 2 debug?
    pub fn at_cmee_e(&mut self, log_level: u8) -> Result<()> {
        self.flush();
        let msg = format!("AT+CMEE={}\r", log_level);
        self.send_command(&msg);
        let out = self.read(Some("OK"), self.timeout_ms, None);
        if !out.ends_with("OK") {
            Err(anyhow!("{}", out))
        } else {
            Ok(())
        }
    }
    /// at+cpin?
    pub fn at_cpin_q(&mut self) -> Result<CPIN> {
        self.flush();
        self.send_command("AT+CPIN?\r");
        let out = self.read(Some("OK"), self.timeout_ms, None);
        if !out.ends_with("OK") {
            Err(anyhow!("got error: {}", out))
        } else {
            if out.contains("READY") {
                Ok(CPIN::READY)
            } else if out.contains("ERROR") {
                Ok(CPIN::ERROR(out))
            } else {
                Ok(CPIN::UNKNOWN(out))
            }
        }
    }

    /// AT+CMGF=X, eg. AT+CMGF=1
    pub fn at_cmgf_e(&mut self, cmgf: u64) -> Result<()> {
        self.flush();
        let cmd = format!("AT+CMGF={}\r", cmgf);
        self.send_command(&cmd);
        let out = self.read(Some("OK"), self.timeout_ms, None);
        if !out.ends_with("OK") {
            Err(anyhow!("got error: {}", out))
        } else {
            Ok(())
        }
    }

    /// AT+CMGS="+31638740161" <ENTER> - send sms control+z \n
    pub fn at_cmgs_e(&mut self, phone_number: &str, msg: &str) -> String {
        self.flush();
        let cmd = format!("AT+CMGS=\"{}\"\r", phone_number);
        self.send_command(&cmd);
        let out = self.read(Some(">"), 1000, None);
        debug!("out {}", out);
        // control-z
        let msg = msg.to_string() + "\u{001A}\n";
        self.send_command(&msg);
        // CONTROL  - Z
        let out = self.read(Some("OK"), self.timeout_ms, None);
        out
    }

    /// AT+CPMS="SM" - to read sms from sim card
    pub fn at_cpms_e_sm(&mut self) -> Result<()> {
        self.send_command("AT+CMPS=\"SM\"\r");
        let out = self.read(Some("OK"), self.timeout_ms, None);
        if !out.ends_with("OK") {
            Err(anyhow!("got error: {}", out))
        } else {
            Ok(())
        }
    }

    /// AT+CMGL="ALL" - to look on sms messages
    pub fn at_cmgl_e_all(&mut self) -> String {
        self.flush();
        self.send_command("AT+CMGL=\"ALL\"\r");
        let out = self.read(Some("OK"), self.timeout_ms, None);
        return out;
    }

    /// AT+SAPBR=3,1,"Contype","GPRS" or AT+SAPBR=3,1,"APN","internet"
    pub fn at_sabr_e(
        &mut self,
        idx_0: u8,
        idx_1: u8,
        key: Option<&str>,
        value: Option<&str>,
    ) -> String {
        self.flush();
        let msg = if let (Some(key), Some(value)) = (key, value) {
            format!("AT+SAPBR={},{},\"{}\",\"{}\"\r", idx_0, idx_1, key, value)
        } else {
            format!("AT+SAPBR={},{}\r", idx_0, idx_1)
        };
        self.send_command(&msg);
        self.read(Some("OK"), self.timeout_ms, None)
    }

    /// HTTPINIT Check the HTTP connection status
    pub fn at_httpinit(&mut self) -> Result<()> {
        self.flush();
        self.send_command("AT+HTTPINIT\r");
        let out = self.read(Some("OK"), self.timeout_ms, None);
        if !out.ends_with("OK") {
            Err(anyhow!("got error: {}", out))
        } else {
            Ok(())
        }
    }

    ///  Set parameters for HTTP session AT+HTTPPARA="CID",1 or AT+HTTPPARA="URL","www.sim.com"
    pub fn at_httppara_e(&mut self, key: &str, value: HTTPPARA) -> Result<()> {
        self.flush();
        let mut msg = String::new();
        match value {
            HTTPPARA::S(value) => {
                msg = format!("AT+HTTPPARA=\"{}\",\"{}\"\r", key, value);
            }
            HTTPPARA::I(value) => {
                msg = format!("AT+HTTPPARA=\"{}\",{}\r", key, value);
            }
        }
        self.send_command(&msg);
        let out = self.read(Some("OK"), self.timeout_ms, None);
        if !out.ends_with("OK") {
            Err(anyhow!("got error: {}", out))
        } else {
            Ok(())
        }
    }

    /// AT+HTTPSSL=X Set ssl - on 1 off 0
    pub fn at_httpssl_e(&mut self, value: u8) -> Result<()> {
        self.flush();
        let msg = format!("AT+HTTPSSL={}\r", value);
        self.send_command(&msg);
        let out = self.read(Some("OK"), self.timeout_ms, None);
        if !out.ends_with("OK") {
            Err(anyhow!("got error: {}", out))
        } else {
            Ok(())
        }
    }

    // AT+HTTPSSL?
    pub fn at_httpssl_q(&mut self) -> HTTPS {
        self.flush();
        self.send_command("AT+HTTPSSL?\r");
        let out = self.read(Some("OK"), self.timeout_ms, None);
        if out.contains("HTTPSSL: 1") {
            HTTPS::ON
        } else {
            HTTPS::OFF
        }
    }

    /// AT+HTTPACTION=X - get sessin start
    pub fn at_httpaction_e(&mut self, http_action: HTTP_ACTION) -> Result<()> {
        self.flush();
        let msg = format!("AT+HTTPACTION={}\r", http_action as u8);
        self.send_command(&msg);
        let out = self.read(Some("OK"), self.timeout_ms, None);
        if !out.ends_with("OK") {
            Err(anyhow!("got error: {}", out))
        } else {
            Ok(())
        }
    }

    /// AT+HTTPREAD Read the data of the HTTP server
    pub fn at_httpread(&mut self) -> String {
        self.flush();
        self.send_command("AT+HTTPREAD\r");
        self.read(Some("OK"), self.timeout_ms, None)
    }

    /// AT+HTTPTERM  End HTTP service
    pub fn at_httpterm(&mut self) -> Result<()> {
        self.flush();
        self.send_command("AT+HTTPTERM\r");
        let out = self.read(Some("OK"), self.timeout_ms, None);
        if !out.ends_with("OK") {
            Err(anyhow!("got error: {}", out))
        } else {
            Ok(())
        }
    }
}

// http + https
impl Sim800C {
    pub fn set_gprs(&mut self) -> Result<()> {
        let out = self.at_sabr_e(3, 1, Some("Contype"), Some("GPRS"));
        if !out.ends_with("OK") {
            Err(anyhow!("got error: {}", out))
        } else {
            Ok(())
        }
    }

    pub fn set_apn(&mut self) -> Result<()> {
        let apn = self.apn.to_string();
        let out = self.at_sabr_e(3, 1, Some("APN"), Some(&apn));
        if !out.ends_with("OK") {
            Err(anyhow!("got error: {}", out))
        } else {
            Ok(())
        }
    }

    pub fn open_gprs_context(&mut self) -> Result<()> {
        let out = self.at_sabr_e(1, 1, None, None);
        if !out.ends_with("OK") {
            Err(anyhow!("got error: {}", out))
        } else {
            Ok(())
        }
    }

    pub fn query_gprs_context(&mut self) -> String {
        self.at_sabr_e(2, 1, None, None)
    }

    pub fn close_gprs_context(&mut self) -> Result<()> {
        let out = self.at_sabr_e(0, 1, None, None);
        if !out.ends_with("OK") {
            Err(anyhow!("got error: {}", out))
        } else {
            Ok(())
        }
    }

    /// checks if https is off and start if it is not already running
    pub fn https_on(&mut self) -> Result<()> {
        if self.at_httpssl_q() == HTTPS::OFF {
            self.at_httpssl_e(1)?
        }
        Ok(())
    }

    pub fn https_off(&mut self) -> Result<()> {
        if self.at_httpssl_q() == HTTPS::ON {
            self.at_httpssl_e(0)?
        }
        Ok(())
    }
}

//
// AT+SAPBR=3,1,"Contype","GPRS"
// OK
// AT+SAPBR=3,1,"APN","internet"
// OK
// AT+SAPBR=2,1
// +SAPBR: 1,3,"0.0.0.0"
//
// OK
// AT+SAPBR=1,1
// OK
// AT+SAPBR=2,1
// +SAPBR: 1,1,"10.213.35.77"
//
// AT+HTTPINIT
// AT+HTTPPARA="CID",1
// AT+HTTPPARA="URL","http://webhook.site/f101b8fc-4597-4b24-8987-18e779388171"
// AT+HTTPACTION=1
// AT+HTTPREAD
// AT+HTTPTERM
//
// AT+CGATT?
// AT+CIPMODE=1
// AT+CSTT="internet"
// AT+CIICR
// AT+CIFSR
//
// HTTPS
// /*
// AT+HTTPINIT
//
// OK
// Init HTTP service
// AT+HTTPPARA="CID",1
// OK
// AT+HTTPPARA="URL","www.gmail.com"
// OK
// AT+HTTPPARA ="REDIR",1
// OK
// Set parameters for HTTP session
// AT+HTTPSSL=1
// OK
// Enable HTTPS function
// AT+HTTPACTION=0
// OK
// GET session start
// +HTTPACTION: 0,200,84200 GET successfully
// AT+HTTPREAD
// +HTTPREAD: 84200
// at+httpssl?
// AT+HTTPTERM
