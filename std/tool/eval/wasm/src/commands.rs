use crate::{ Advice, AdviceLevel };

#[cfg_attr(not(feature = "wasm"), allow(dead_code))]
pub fn test() -> Vec<Advice> {
    vec![
        Advice::new(
            AdviceLevel::Warning,
            "Warning 1".to_string(),
            vec![
                "AAAAAAAAAA".to_string(),
                "AAAAAAAAAA".to_string(),
                "AAAAAAAAAA".to_string(),
                "AAAAAAAAAA".to_string(),
            ]
        ),
        Advice::new(
            AdviceLevel::Warning,
            "Warning 2".to_string(),
            vec![
                "AAAAAAAAAA".to_string(),
                "AAAAAAAAAA".to_string(),
                "AAAAAAAAAA".to_string(),
                "AAAAAAAAAA".to_string(),
            ]
        ),
        Advice::new(
            AdviceLevel::Info,
            "Info 1".to_string(),
            vec![
                "AAAAAAAAAA".to_string(),
                "AAAAAAAAAA".to_string(),
                "AAAAAAAAAA".to_string(),
                "AAAAAAAAAA".to_string(),
            ]
        )
    ]
}
