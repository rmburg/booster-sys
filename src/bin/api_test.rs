use cxx::let_cxx_string;

use booster_sys::ffi::*;

fn main() {
    let api_id = LocoApiId::kWaveHand;
    let_cxx_string!(param = "");
    let mut client = b1_loco_client_new();

    client.pin_mut().Init();
    client.pin_mut().SendApiRequest(api_id, &param);
}
