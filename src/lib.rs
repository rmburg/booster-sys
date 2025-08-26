#[cxx::bridge(namespace = "booster::robot::b1")]
pub mod ffi {
    #[repr(u32)]
    enum LocoApiId {
        kChangeMode = 2000,
        kMove = 2001,
        kRotateHead = 2004,
        kWaveHand = 2005,
        kRotateHeadWithDirection = 2006,
        kLieDown = 2007,
        kGetUp = 2008,
        kMoveHandEndEffector = 2009,
        kControlGripper = 2010,
        kGetFrameTransform = 2011,
        kSwitchHandEndEffectorControlMode = 2012,
        kControlDexterousHand = 2013,
        kHandshake = 2015,
        kDance = 2016,
        kGetMode = 2017,
    }

    unsafe extern "C++" {
        include!("booster/robot/b1/b1_loco_client.hpp");
        include!("wrapper.hpp");

        type B1LocoClient;
        type LocoApiId;

        fn SendApiRequest(
            self: Pin<&mut B1LocoClient>,
            api_id: LocoApiId,
            param: &CxxString,
        ) -> i32;

        fn Init(self: Pin<&mut B1LocoClient>);

        #[namespace = ""]
        #[cxx_name = "construct_unique"]
        fn b1_loco_client_new() -> UniquePtr<B1LocoClient>;
    }
}

#[cfg(test)]
mod tests {
    use cxx::let_cxx_string;

    use crate::ffi::{LocoApiId, b1_loco_client_new};

    #[test]
    fn api_call() {
        let api_id = LocoApiId::kWaveHand;
        let_cxx_string!(param = "");
        let mut client = b1_loco_client_new();

        client.pin_mut().Init();
        client.pin_mut().SendApiRequest(api_id, &param);
    }
}
