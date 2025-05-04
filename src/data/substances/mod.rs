mod logic;
mod effects_processing;

use crate::data::effects::Effect;

/// Macro for quick struct definition and price setting
macro_rules! define_substance {
    ($name: tt, $price: expr) => {
        #[derive(Copy, Clone, Debug, PartialEq)]
        pub(crate) struct $name(f32);

        impl $name {
            pub(crate) fn new() -> Self {
                Self($price)
            }
            pub(crate) fn get_price(&self) -> f32 {
                self.0
            }
        }

        // impl ProcessableSubstance for $name {}
    }
}

// pub(crate) trait ProcessableSubstance: Mixable {
//     fn get_cost(&self) -> f32 {
//         Mixable::get_cost(self)
//     }
//     fn get_replacement_effect(&self, effect: &Effect) -> Option<Effect> {
//         Mixable::get_replacement_effect(self, effect)
//     }
//     fn get_default_effect(&self) -> Effect {
//         Mixable::get_default_effect(self)
//     }
// }

trait Mixable {
    fn get_cost(&self) -> f32;
    fn get_replacement_effect(&self, effect: &Effect) -> Option<Effect>;
    fn get_default_effect(&self) -> Effect;
}


#[derive(Debug, strum::Display, PartialEq)]
pub(crate) enum Substance {
    Addy(Addy),
    Banana(Banana),
    Battery(Battery),
    Chili(Chili),
    Cuke(Cuke),
    Donut(Donut),
    EnergyDrink(EnergyDrink),
    FluMedicine(FluMedicine),
    Gasoline(Gasoline),
    HorseSemen(HorseSemen),
    Iodine(Iodine),
    MegaBean(MegaBean),
    MotorOil(MotorOil),
    MouthWash(MouthWash),
    Paracetamol(Paracetamol),
    Viagor(Viagor),
}

//  Tupled value in the struct is the item's price
define_substance!(Addy, 9.0);
define_substance!(Banana, 2.0);
define_substance!(Battery, 8.0);
define_substance!(Chili, 7.0);
define_substance!(Cuke, 2.0);
define_substance!(Donut, 3.0);
define_substance!(EnergyDrink, 6.0);
define_substance!(FluMedicine, 5.0);
define_substance!(Gasoline, 5.0);
define_substance!(HorseSemen, 9.0);
define_substance!(Iodine, 8.0);
define_substance!(MegaBean, 7.0);
define_substance!(MotorOil, 6.0);
define_substance!(MouthWash, 4.0);
define_substance!(Paracetamol, 3.0);
define_substance!(Viagor, 4.0);

impl Substance {
    pub(crate) fn get_cost(&self) -> f32 {
        match self {
            Self::Addy(substance) => substance.get_cost(),
            Self::Banana(substance) => substance.get_cost(),
            Self::Battery(substance) => substance.get_cost(),
            Self::Chili(substance) => substance.get_cost(),
            Self::Cuke(substance) => substance.get_cost(),
            Self::Donut(substance) => substance.get_cost(),
            Self::EnergyDrink(substance) => substance.get_cost(),
            Self::FluMedicine(substance) => substance.get_cost(),
            Self::Gasoline(substance) => substance.get_cost(),
            Self::HorseSemen(substance) => substance.get_cost(),
            Self::Iodine(substance) => substance.get_cost(),
            Self::MegaBean(substance) => substance.get_cost(),
            Self::MotorOil(substance) => substance.get_cost(),
            Self::MouthWash(substance) => substance.get_cost(),
            Self::Paracetamol(substance) => substance.get_cost(),
            Self::Viagor(substance) => substance.get_cost(),
        }
    }

    pub(crate) fn try_from_str(value: &str) -> Result<Self, String> {
        match value {
            "Addy" => Ok(Self::Addy(Addy::new())),
            "Banana" => Ok(Self::Banana(Banana::new())),
            "Battery" => Ok(Self::Battery(Battery::new())),
            "Chili" => Ok(Self::Chili(Chili::new())),
            "Cuke" => Ok(Self::Cuke(Cuke::new())),
            "Donut" => Ok(Self::Donut(Donut::new())),
            "Energy Drink" => Ok(Self::EnergyDrink(EnergyDrink::new())),
            "Flu Medicine" => Ok(Self::FluMedicine(FluMedicine::new())),
            "Gasoline" => Ok(Self::Gasoline(Gasoline::new())),
            "Horse Semen" => Ok(Self::HorseSemen(HorseSemen::new())),
            "Iodine" => Ok(Self::Iodine(Iodine::new())),
            "Mega Bean" => Ok(Self::MegaBean(MegaBean::new())),
            "Motor Oil" => Ok(Self::MotorOil(MotorOil::new())),
            "Mouth Wash" => Ok(Self::MouthWash(MouthWash::new())),
            "Paracetamol" => Ok(Self::Paracetamol(Paracetamol::new())),
            "Viagra" => Ok(Self::Viagor(Viagor::new())),
            value => Err(format!("Unknown substance: {}", value))
        }
    }

    pub(crate) fn get_replacement_effect(&self, replacement: &Effect) -> Option<Effect> {
        match self {
            Self::Addy(substance) => substance.get_replacement_effect(replacement),
            Self::Banana(substance) => substance.get_replacement_effect(replacement),
            Self::Battery(substance) => substance.get_replacement_effect(replacement),
            Self::Chili(substance) => substance.get_replacement_effect(replacement),
            Self::Cuke(substance) => substance.get_replacement_effect(replacement),
            Self::Donut(substance) => substance.get_replacement_effect(replacement),
            Self::EnergyDrink(substance) => substance.get_replacement_effect(replacement),
            Self::FluMedicine(substance) => substance.get_replacement_effect(replacement),
            Self::Gasoline(substance) => substance.get_replacement_effect(replacement),
            Self::HorseSemen(substance) => substance.get_replacement_effect(replacement),
            Self::Iodine(substance) => substance.get_replacement_effect(replacement),
            Self::MegaBean(substance) => substance.get_replacement_effect(replacement),
            Self::MotorOil(substance) => substance.get_replacement_effect(replacement),
            Self::MouthWash(substance) => substance.get_replacement_effect(replacement),
            Self::Paracetamol(substance) => substance.get_replacement_effect(replacement),
            Self::Viagor(substance) => substance.get_replacement_effect(replacement),
        }
    }

    pub(crate) fn get_default_effect(&self) -> Effect {
        match self {
            Self::Addy(substance) => substance.get_default_effect(),
            Self::Banana(substance) => substance.get_default_effect(),
            Self::Battery(substance) => substance.get_default_effect(),
            Self::Chili(substance) => substance.get_default_effect(),
            Self::Cuke(substance) => substance.get_default_effect(),
            Self::Donut(substance) => substance.get_default_effect(),
            Self::EnergyDrink(substance) => substance.get_default_effect(),
            Self::FluMedicine(substance) => substance.get_default_effect(),
            Self::Gasoline(substance) => substance.get_default_effect(),
            Self::HorseSemen(substance) => substance.get_default_effect(),
            Self::Iodine(substance) => substance.get_default_effect(),
            Self::MegaBean(substance) => substance.get_default_effect(),
            Self::MotorOil(substance) => substance.get_default_effect(),
            Self::MouthWash(substance) => substance.get_default_effect(),
            Self::Paracetamol(substance) => substance.get_default_effect(),
            Self::Viagor(substance) => substance.get_default_effect(),
        }
    }
}
