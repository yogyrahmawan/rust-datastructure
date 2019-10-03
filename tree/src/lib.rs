
#[derive(Clone, Debug)]
pub struct IoTDevice {
    pub numerical_id: u64,
    pub path: String,
    pub address: String,
}

impl IoTDevice {
    pub fn new(id: u64, address: impl Into<String>, path: impl Into<String>) -> IoTDevice {
        IoTDevice {
            address: address.into(),
            numerical_id: id,
            path: path.into(),
        }
    }
}