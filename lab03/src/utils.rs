use embassy_rp::pwm::Config as ConfigPwm;

pub enum Lane {
    LaneA,
    LaneB,
    Both,
}

const TOP_PWM_VAL: u16 = 0x9088;

pub fn creeate_pwm_config(ratio: u16, lane: Lane) -> ConfigPwm {
    let mut config: ConfigPwm = Default::default();

    config.top = TOP_PWM_VAL;

    let ratio_inverted;
    if ratio > 100 {
        ratio_inverted = 0; // avoid overflow to prevent panicking
    } else {
        ratio_inverted = 100 - ratio;
    }
    let compare: u16 = ((TOP_PWM_VAL as u32 * ratio_inverted as u32) / 100) as u16;

    match lane {
        Lane::LaneA => config.compare_a = compare,
        Lane::LaneB => config.compare_b = compare,
        Lane::Both => {
            config.compare_a = compare;
            config.compare_b = compare;
        }
    }

    config
}

pub fn update_pwm_config(config: &mut ConfigPwm, ratio: u16, lane: Lane) {
    let ratio_inverted;
    if ratio > 100 {
        ratio_inverted = 0; // avoid overflow to prevent programm panicking
    } else {
        ratio_inverted = 100 - ratio;
    }
    let compare: u16 = ((TOP_PWM_VAL as u32 * ratio_inverted as u32) / 100) as u16;

    match lane {
        Lane::LaneA => config.compare_a = compare,
        Lane::LaneB => config.compare_b = compare,
        Lane::Both => {
            config.compare_a = compare;
            config.compare_b = compare;
        }
    }
}