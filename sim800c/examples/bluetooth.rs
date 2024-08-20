

pub fn main(){
    // AT+BTPOWER=1 	Turn on the Bluetooth power 	OK
    // AT+BTHOST? 	Query Bluetooth name and address 	Returns the bluetooth name and MAC address
    // AT+BTSCAN=1,10 	Search for nearby Bluetooth devices 	Returns the searched Bluetooth device information
    // AT+BTPAIR=1,1 	Digital confirmation mode response pairing 	OK
    // AT+BTACPT=1 	Accept client connection request 	OK
    // AT+BTSPPSEND 	Send data 	Return > to start entering data,
    //
    // Send hexadecimal 1A to end sending
    // AT+BTPOWER=0 	Turn off the Bluetooth power 	OK
}