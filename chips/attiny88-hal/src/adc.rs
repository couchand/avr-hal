extern crate avr_hal_generic as avr_hal;

use crate::port::porta::{PA0, PA1};
use crate::port::portc::{PC0, PC1, PC2, PC3, PC4, PC5};

use crate::attiny88::adc::admux::MUX_A;

avr_hal::impl_adc! {
    pub struct Adc {
        type ChannelID = MUX_A;
        peripheral: crate::attiny88::ADC,
        set_mux: |peripheral, id| {
            peripheral.admux.modify(|_, w| w.mux().variant(id));
        },
        set_ref: |peripheral, voltage| {
            peripheral.admux.write(|w| match voltage {
                ReferenceVoltage::Internal => w.refs0().internal(),
                ReferenceVoltage::AVcc => w.refs0().avcc(),
                ReferenceVoltage::Aref => panic!(),
            });
        },
        pins: {
            pc0: (PC0, MUX_A::ADC0, didr0::adc0d),
            pc1: (PC1, MUX_A::ADC1, didr0::adc1d),
            pc2: (PC2, MUX_A::ADC2, didr0::adc2d),
            pc3: (PC3, MUX_A::ADC3, didr0::adc3d),
            pc4: (PC4, MUX_A::ADC4, didr0::adc4d),
            pc5: (PC5, MUX_A::ADC5, didr0::adc5d),
            pa0: (PA0, MUX_A::ADC6, didr0::adc6d),
            pa1: (PA1, MUX_A::ADC7, didr0::adc7d),
        }
    }
}

/// Additional channels
///
/// This module contains ADC channels, additional to the direct pin channels.
pub mod channel {
    use avr_hal::hal::adc::Channel;
    use crate::attiny88::adc::admux::MUX_A;

    /// Channel for the _Bandgap Reference Voltage_
    pub struct Vbg;
    impl Channel<super::Adc> for Vbg {
        type ID = MUX_A;
        fn channel() -> Self::ID { MUX_A::ADC_VBG }
    }

    /// Channel for _GND_
    pub struct Gnd;
    impl Channel<super::Adc> for Gnd {
        type ID = MUX_A;
        fn channel() -> Self::ID { MUX_A::ADC_GND }
    }

    /// Channel for the built-in _Temperature Sensor_
    pub struct Temperature;
    impl Channel<super::Adc> for Temperature {
        type ID = MUX_A;
        fn channel() -> Self::ID { MUX_A::TEMPSENS }
    }
}
