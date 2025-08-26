use cxx::let_cxx_string;

use booster_sys::robot::{b1::ffi::{b1_loco_client_new, LocoApiId}, ffi::init_channel_factory};

fn main() {
    let_cxx_string!(network_interface = "127.0.0.1");
    init_channel_factory(&network_interface);
    
    let api_id = LocoApiId::kWaveHand;
    let_cxx_string!(param = "");
    let mut client = b1_loco_client_new();

    client.pin_mut().Init();
    client.pin_mut().SendApiRequest(api_id, &param);
}
