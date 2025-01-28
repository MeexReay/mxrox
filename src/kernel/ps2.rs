const DATA_PORT: *mut u8 = 0x60 as *mut u8;
const STATUS_PORT: *mut u8 = 0x64 as *mut u8;

fn write_ps2_data(data: u8) {
    todo!()
} 

fn read_ps2_data() -> u8 {
    todo!()
} 

fn send_ps2_command(command: u8) {
    todo!()
} 

fn read_ps2_status() -> u8 {
    todo!()
} 

/// returns device type bytes
fn init_ps2_controller() -> vec![u8; 2] {
    todo!()
}