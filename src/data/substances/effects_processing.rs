use crate::data::effects::Effect;

use super::{Addy, Banana, Battery, Chili, Cuke, Donut, EnergyDrink, FluMedicine, Gasoline, HorseSemen, Iodine, MegaBean, Mixable, MotorOil, MouthWash, Paracetamol, Viagor};

//-----------------------------------------------------------------------------------------------
//  Implementations
//-----------------------------------------------------------------------------------------------
impl Mixable for Addy {
    fn get_cost(&self) -> f32 {
        self.get_price()
    }
    fn get_replacement_effect(&self, effect: &Effect) -> Option<Effect> {
        match effect {
            Effect::Explosive => Some(Effect::Euphoric),
            Effect::Foggy => Some(Effect::Energizing),
            Effect::Glowing => Some(Effect::Refreshing),
            Effect::LongFaced => Some(Effect::Electrifying),
            Effect::Sedating => Some(Effect::Gingeritis),
            _ => None,
        }
    }
    fn get_default_effect(&self) -> Effect {
        Effect::ThoughtProvoking
    }
}

impl Mixable for Banana {
    fn get_cost(&self) -> f32 {
        self.get_price()
    }
    fn get_replacement_effect(&self, effect: &Effect) -> Option<Effect> {
        match effect {
            Effect::Calming => Some(Effect::Sneaky),
            Effect::Cyclopean => Some(Effect::Energizing),
            Effect::Disorienting => Some(Effect::Focused),
            Effect::Energizing => Some(Effect::ThoughtProvoking),
            Effect::Focused => Some(Effect::SeizureInducing),
            Effect::LongFaced => Some(Effect::Refreshing),
            Effect::Paranoia => Some(Effect::Jennerising),
            Effect::Smelly => Some(Effect::AntiGravity),
            Effect::Toxic => Some(Effect::Smelly),
            _ => None,
        }
    }
    fn get_default_effect(&self) -> Effect {
        Effect::Gingeritis
    }
}

impl Mixable for Battery {
    fn get_cost(&self) -> f32 {
        self.get_price()
    }
    fn get_replacement_effect(&self, effect: &Effect) -> Option<Effect> {
        match effect {
            Effect::Cyclopean => Some(Effect::Glowing),
            Effect::Electrifying => Some(Effect::Euphoric),
            Effect::Euphoric => Some(Effect::Zombifying),
            Effect::Laxative => Some(Effect::CalorieDense),
            Effect::Munchies => Some(Effect::TropicThunder),
            Effect::Shrinking => Some(Effect::Munchies),
            _ => None,
        }
    }
    fn get_default_effect(&self) -> Effect {
        Effect::BrightEyed
    }
}

impl Mixable for Chili {
    fn get_cost(&self) -> f32 {
        self.get_price()
    }
    fn get_replacement_effect(&self, effect: &Effect) -> Option<Effect> {
        match effect {
            Effect::AntiGravity => Some(Effect::TropicThunder),
            Effect::Athletic => Some(Effect::Euphoric),
            Effect::Laxative => Some(Effect::LongFaced),
            Effect::Munchies => Some(Effect::Toxic),
            Effect::Shrinking => Some(Effect::Refreshing),
            Effect::Sneaky => Some(Effect::BrightEyed),
            _ => None,
        }
    }
    fn get_default_effect(&self) -> Effect {
        Effect::Spicy
    }
}

impl Mixable for Cuke {
    fn get_cost(&self) -> f32 {
        self.get_price()
    }
    fn get_replacement_effect(&self, effect: &Effect) -> Option<Effect> {
        match effect {
            Effect::Euphoric => Some(Effect::Laxative),
            Effect::Foggy => Some(Effect::Cyclopean),
            Effect::Gingeritis => Some(Effect::ThoughtProvoking),
            Effect::Munchies => Some(Effect::Athletic),
            Effect::Slippery => Some(Effect::Munchies),
            Effect::Sneaky => Some(Effect::Paranoia),
            Effect::Toxic => Some(Effect::Euphoric),
            _ => None,
        }
    }
    fn get_default_effect(&self) -> Effect {
        Effect::Energizing
    }
}

impl Mixable for Donut {
    fn get_cost(&self) -> f32 {
        self.get_price()
    }
    fn get_replacement_effect(&self, effect: &Effect) -> Option<Effect> {
        match effect {
            Effect::AntiGravity => Some(Effect::Slippery),
            Effect::Balding => Some(Effect::Sneaky),
            Effect::CalorieDense => Some(Effect::Explosive),
            Effect::Focused => Some(Effect::Euphoric),
            Effect::Jennerising => Some(Effect::Gingeritis),
            Effect::Munchies => Some(Effect::Calming),
            Effect::Shrinking => Some(Effect::Energizing),
            _ => None,
        }
    }
    fn get_default_effect(&self) -> Effect {
        Effect::CalorieDense
    }
}

impl Mixable for EnergyDrink {
    fn get_cost(&self) -> f32 {
        self.get_price()
    }
    fn get_replacement_effect(&self, effect: &Effect) -> Option<Effect> {
        match effect {
            Effect::Disorienting => Some(Effect::Electrifying),
            Effect::Euphoric => Some(Effect::Energizing),
            Effect::Focused => Some(Effect::Shrinking),
            Effect::Foggy => Some(Effect::Laxative),
            Effect::Glowing => Some(Effect::Disorienting),
            Effect::Schizophrenia => Some(Effect::Balding),
            Effect::Sedating => Some(Effect::Munchies),
            Effect::Spicy => Some(Effect::Euphoric),
            Effect::TropicThunder => Some(Effect::Sneaky),
            _ => None,
        }
    }
    fn get_default_effect(&self) -> Effect {
        Effect::Athletic
    }
}

impl Mixable for FluMedicine {
    fn get_cost(&self) -> f32 {
        self.get_price()
    }
    fn get_replacement_effect(&self, effect: &Effect) -> Option<Effect> {
        match effect {
            Effect::Athletic => Some(Effect::Munchies),
            Effect::Calming => Some(Effect::BrightEyed),
            Effect::Cyclopean => Some(Effect::Foggy),
            Effect::Electrifying => Some(Effect::Refreshing),
            Effect::Euphoric => Some(Effect::Toxic),
            Effect::Focused => Some(Effect::Calming),
            Effect::Laxative => Some(Effect::Euphoric),
            Effect::Munchies => Some(Effect::Slippery),
            Effect::Shrinking => Some(Effect::Paranoia),
            Effect::ThoughtProvoking => Some(Effect::Gingeritis),
            _ => None,
        }
    }
    fn get_default_effect(&self) -> Effect {
        Effect::Sedating
    }
}

impl Mixable for Gasoline {
    fn get_cost(&self) -> f32 {
        self.get_price()
    }
    fn get_replacement_effect(&self, effect: &Effect) -> Option<Effect> {
        match effect {
            Effect::Disorienting => Some(Effect::Glowing),
            Effect::Electrifying => Some(Effect::Disorienting),
            Effect::Energizing => Some(Effect::Euphoric),
            Effect::Euphoric => Some(Effect::Spicy),
            Effect::Gingeritis => Some(Effect::Smelly),
            Effect::Jennerising => Some(Effect::Sneaky),
            Effect::Laxative => Some(Effect::Foggy),
            Effect::Munchies => Some(Effect::Sedating),
            Effect::Paranoia => Some(Effect::Calming),
            Effect::Shrinking => Some(Effect::Focused),
            Effect::Sneaky => Some(Effect::TropicThunder),
            _ => None,
        }
    }
    fn get_default_effect(&self) -> Effect {
        Effect::Toxic
    }
}

impl Mixable for HorseSemen {
    fn get_cost(&self) -> f32 {
        self.get_price()
    }
    fn get_replacement_effect(&self, effect: &Effect) -> Option<Effect> {
        match effect {
            Effect::AntiGravity => Some(Effect::Calming),
            Effect::Gingeritis => Some(Effect::Refreshing),
            Effect::SeizureInducing => Some(Effect::Energizing),
            Effect::ThoughtProvoking => Some(Effect::Electrifying),
            _ => None,
        }
    }
    fn get_default_effect(&self) -> Effect {
        Effect::LongFaced
    }
}

impl Mixable for Iodine {
    fn get_cost(&self) -> f32 {
        self.get_price()
    }
    fn get_replacement_effect(&self, effect: &Effect) -> Option<Effect> {
        match effect {
            Effect::Calming => Some(Effect::Balding),
            Effect::CalorieDense => Some(Effect::Gingeritis),
            Effect::Euphoric => Some(Effect::SeizureInducing),
            Effect::Foggy => Some(Effect::Paranoia),
            Effect::Refreshing => Some(Effect::ThoughtProvoking),
            Effect::Toxic => Some(Effect::Sneaky),
            _ => None,
        }
    }
    fn get_default_effect(&self) -> Effect {
        Effect::Jennerising
    }
}

impl Mixable for MegaBean {
    fn get_cost(&self) -> f32 {
        self.get_price()
    }
    fn get_replacement_effect(&self, effect: &Effect) -> Option<Effect> {
        match effect {
            Effect::Athletic => Some(Effect::Laxative),
            Effect::Calming => Some(Effect::Glowing),
            Effect::Energizing => Some(Effect::Cyclopean),
            Effect::Focused => Some(Effect::Disorienting),
            Effect::Jennerising => Some(Effect::Paranoia),
            Effect::SeizureInducing => Some(Effect::Focused),
            Effect::Shrinking => Some(Effect::Electrifying),
            Effect::Slippery => Some(Effect::Toxic),
            Effect::Sneaky => Some(Effect::Calming),
            Effect::ThoughtProvoking => Some(Effect::Energizing),
            _ => None,
        }
    }
    fn get_default_effect(&self) -> Effect {
        Effect::Foggy
    }
}

impl Mixable for MotorOil {
    fn get_cost(&self) -> f32 {
        self.get_price()
    }
    fn get_replacement_effect(&self, effect: &Effect) -> Option<Effect> {
        match effect {
            Effect::Energizing => Some(Effect::Munchies),
            Effect::Euphoric => Some(Effect::Sedating),
            Effect::Foggy => Some(Effect::Toxic),
            Effect::Munchies => Some(Effect::Schizophrenia),
            Effect::Paranoia => Some(Effect::AntiGravity),
            _ => None,
        }
    }
    fn get_default_effect(&self) -> Effect {
        Effect::Slippery
    }
}

impl Mixable for MouthWash {
    fn get_cost(&self) -> f32 {
        self.get_price()
    }
    fn get_replacement_effect(&self, effect: &Effect) -> Option<Effect> {
        match effect {
            Effect::Calming => Some(Effect::AntiGravity),
            Effect::CalorieDense => Some(Effect::Sneaky),
            Effect::Explosive => Some(Effect::Sedating),
            Effect::Focused => Some(Effect::Jennerising),
            _ => None,
        }
    }
    fn get_default_effect(&self) -> Effect {
        Effect::Balding
    }
}

impl Mixable for Paracetamol {
    fn get_cost(&self) -> f32 {
        self.get_price()
    }
    fn get_replacement_effect(&self, effect: &Effect) -> Option<Effect> {
        match effect {
            Effect::Calming => Some(Effect::Slippery),
            Effect::Electrifying => Some(Effect::Athletic),
            Effect::Energizing => Some(Effect::Paranoia),
            Effect::Focused => Some(Effect::Gingeritis),
            Effect::Foggy => Some(Effect::Calming),
            Effect::Glowing => Some(Effect::Toxic),
            Effect::Munchies => Some(Effect::AntiGravity),
            Effect::Paranoia => Some(Effect::Balding),
            Effect::Spicy => Some(Effect::BrightEyed),
            Effect::Toxic => Some(Effect::TropicThunder),
            _ => None,
        }
    }
    fn get_default_effect(&self) -> Effect {
        Effect::Sneaky
    }
}

impl Mixable for Viagor {
    fn get_cost(&self) -> f32 {
        self.get_price()
    }
    fn get_replacement_effect(&self, effect: &Effect) -> Option<Effect> {
        match effect {
            Effect::Athletic => Some(Effect::Sneaky),
            Effect::Disorienting => Some(Effect::Toxic),
            Effect::Euphoric => Some(Effect::BrightEyed),
            Effect::Laxative => Some(Effect::Calming),
            Effect::Shrinking => Some(Effect::Gingeritis),
            _ => None,
        }
        
    }
    fn get_default_effect(&self) -> Effect {
        Effect::TropicThunder
    }
}
