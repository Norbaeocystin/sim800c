
fn main(){
    // AT+CGATT? 	Check GPRS attachment service status 	OK
    // AT+CIPMODE=1 	Set to transparent mode 	OK
    // AT+CSTT="CMNET" 	Set APN 	OK
    // AT+CIICR 	Establish a wireless connection 	OK
    // AT+CIFSR 	Retrieve the local IP address 	OK
    // AT+CIPSTART="TCP","118.190.93.84",2317 	Establish a TCP client connection 	OK
    // AT+CIPSTART="UDP","118.190.93.84",2317 	Establish a UDP client connection 	OK
    // AT+CIPSEND=<string length> 	Send a specified string length 	OK
    // AT+CIPCLOSE 	Close the connection 	OK
    // AT+CIPSHUT 	Close PDP context connection 	OK
}