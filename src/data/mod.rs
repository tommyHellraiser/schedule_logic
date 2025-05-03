mod substances;
pub(crate) mod effects;

use effects::Effect;
use substances::Substance;

#[derive(Debug, PartialEq)]
pub(crate) struct Mix {
    product: Product,
    substances: Vec<Substance>,
    effects: Vec<Effect>,
    cost: f32,
    sell_price: f32
}

impl Mix {
    #[cfg(test)]
    pub(crate) fn new_instance(
        product: &str,
        substances: Vec<&str>,
        effects: Vec<Effect>,
        cost: f32,
        sell_price: f32
    ) -> Result<Self, String> {
        Ok(Self {
            product: Product::try_from_str(product)?,
            substances: substances
                .into_iter()
                .map(Substance::try_from_str)
                .collect::<Result<Vec<Substance>, String>>()?,
            effects,
            cost,
            sell_price
        })
    }
    pub(crate) fn new_mix(product: &str, substances: Vec<&str>) -> Result<Self, String> {

        let product = Product::try_from_str(product)?;

        let substances: Result<Vec<Substance>, String> = substances
            .into_iter()
            .map(Substance::try_from_str)
            .collect();
        //  Start the mixing process with a defaulted sell price according to the product,
        // a zero cost (no substances have been added yet), and no effects
        let mut mix_process = Mix::start_process(product);
    
        mix_process.process_substances(substances?);
    
        Ok(mix_process)
    }

    pub(crate) fn start_process(
        product: Product,
    ) -> Self {
        //  Everything defaulted except the sell price
        Self {
            sell_price: product.get_base_price(),
            product,
            substances: vec![],
            effects: vec![],
            cost: 0.0,
        }
    }

    /// Main function used to calcuate and process the mix
    pub(crate) fn process_substances(&mut self, substances: Vec<Substance>) {
        //  First, add the base effect for the product. If it has no additional effects, skip this step
        if let Some(effect) = self.product.get_base_effect() {
            self.effects.push(effect);
        }

        //  Then determine the cost and added effect for each new substance
        for substance in &substances {
            self.cost += substance.get_cost();
            //  We need the substance added in this step to update the effects list and know
            // its sell value. Effects can be replaced by others, so we need to hold this info
            // inside the Substance instance
            self.add_output_effect(substance);
        }

        //  Then assign the substances to the output Mix
        self.substances = substances;

        //  Finally calculate the sell price based on the increments from each resulting effect
        //  Starting at one cause each effect does add compoundly to the sell price, rather they
        // add an increment to the price by summing their factor and then the sell price is
        // the base price of the product multiplied by the sell price factor
        let mut sell_price_factor = 1.0;
        for effect in &self.effects {
            sell_price_factor += effect.get_sell_price_factor();
        }
        self.sell_price = (self.product.get_base_price() * sell_price_factor).round();
    }

    /// Replaces an existing replaceable effect with the new one, or appends it to the array of 
    /// effects if no effect needs to be replaced
    pub(crate) fn add_output_effect(&mut self, substance: &Substance) {
        //  Iterate through all the present effects for this mix
        // and determine if an effect needs to be either replaced or
        // appended from each item's characteristics
        let effects_temp = self.effects.clone();
        for effect in self.effects.iter_mut() {
            if let Some(replacement_effect) = substance.get_replacement_effect(effect) {
                //  If Some is returned, we need to replace the effect and return
                if !effects_temp.contains(&replacement_effect) {
                    *effect = replacement_effect;
                }
            }
        }

        //  If the effects array already contains 8 effects, return and don't add a new one
        if self.effects.len() == 8 {
            return
        }

        //  If none of the effects previously iterated have to be replaced, then
        // append a new one and return
        if !self.effects.contains(&substance.get_default_effect()) {
            self.effects.push(substance.get_default_effect());
        }
    }
}

#[derive(Debug, PartialEq)]
pub(super) enum Product {
    OgKush,
    SourDiesel,
    GreenCrack,
    GrandaddyPurple,
    Meth,
    Cocaine,
}

impl Product {
    pub(crate) fn try_from_str(input: &str) -> Result<Self, String> {
        match input {
            "OG Kush" => Ok(Self::OgKush),
            "Sour Diesel" => Ok(Self::SourDiesel),
            "Green Crack" => Ok(Self::GreenCrack),
            "Grandaddy Purple" => Ok(Self::GrandaddyPurple),
            "Meth" => Ok(Self::Meth),
            "Cocaine" => Ok(Self::Cocaine),
            input => Err(format!("Unknown product: {}", input))
        }
    }

    pub(crate) fn get_base_price(&self) -> f32 {
        match self {
            Self::OgKush => 35.0,
            Self::SourDiesel => 40.0,
            Self::GreenCrack => 46.0,
            Self::GrandaddyPurple => 46.0,
            Self::Meth => 70.0,
            Self::Cocaine => 150.0,
        }
    }

    pub(crate) fn get_base_effect(&self) -> Option<Effect> {
        match self {
            Self::OgKush => Some(Effect::Calming),
            Self::SourDiesel => Some(Effect::Refreshing),
            Self::GreenCrack => Some(Effect::Energizing),
            Self::GrandaddyPurple => Some(Effect::Sedating),
            Self::Meth => None,
            Self::Cocaine => None,
        }
    }
}


