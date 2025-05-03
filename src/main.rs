use data::Mix;

mod data;

fn main() {
    let mix_result = Mix::new_mix
    (
        "Cocaine",
        vec![
            "Motor Oil",
            "Cuke",
            "Paracetamol",
            "Gasoline",
            "Cuke",
            "Battery",
            "Horse Semen",
            "Mega Bean"
        ]
    );

    let mix = mix_result.unwrap();
}

#[cfg(test)]
mod mix_test {
    use crate::Mix;
    use crate::data::effects::Effect;

    #[test]
    fn test_1() {
        let product = "Cocaine";
        let substances = vec![
            "Motor Oil",
            "Cuke",
            "Paracetamol",
            "Gasoline",
            "Cuke",
            "Battery",
            "Horse Semen",
            "Mega Bean"
        ];
        let mix = Mix::new_mix(product, substances.clone()).unwrap();

        let expected_mix = Mix::new_instance(
            "Cocaine",
            substances,
            vec![
                Effect::AntiGravity,
                Effect::Glowing,
                Effect::TropicThunder,
                Effect::Zombifying,
                Effect::Cyclopean,
                Effect::BrightEyed,
                Effect::LongFaced,
                Effect::Foggy
            ],
            42.0,
            735.0
        ).unwrap();

        assert_eq!(mix, expected_mix)
    }

    #[test]
    fn test_2() {
        let product = "OG Kush";
        let substances = vec![
            "Banana",
            "Gasoline",
            "Paracetamol",
            "Cuke",
            "Mega Bean",
            "Battery",
            "Banana",
            "Cuke"
        ];
        let mix = Mix::new_mix(product, substances.clone()).unwrap();

        let expected_mix = Mix::new_instance(
            "OG Kush",
            substances,
            vec![
                Effect::TropicThunder,
                Effect::AntiGravity,
                Effect::Zombifying,
                Effect::Jennerising,
                Effect::Glowing,
                Effect::Cyclopean,
                Effect::BrightEyed,
                Effect::ThoughtProvoking
            ],
            31.0,
            171.0
        ).unwrap();

        assert_eq!(mix, expected_mix)
    }

    #[test]
    fn test_3() {
        let product = "Meth";
        let substances = vec![
            "Banana",
            "Cuke",
            "Paracetamol",
            "Gasoline",
            "Cuke",
            "Battery",
            "Horse Semen",
            "Mega Bean"
        ];
        let mix = Mix::new_mix(product, substances.clone()).unwrap();

        let expected_mix = Mix::new_instance(
            "Meth",
            substances,
            vec![
                Effect::Electrifying,
                Effect::Glowing,
                Effect::TropicThunder,
                Effect::Zombifying,
                Effect::Cyclopean,
                Effect::BrightEyed,
                Effect::LongFaced,
                Effect::Foggy
            ],
            38.0,
            340.0
        ).unwrap();

        assert_eq!(mix, expected_mix)
    }
}
