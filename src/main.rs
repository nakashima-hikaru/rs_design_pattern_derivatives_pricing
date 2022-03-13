pub mod chapter1;
pub mod chapter2;
pub mod chapter3;
pub mod chapter4;
pub mod chapter5;
pub mod chapter6;
pub mod chapter7;
pub mod equity_fx_main;
pub mod random_main3;
pub mod simple_mc_main1;
pub mod simple_mc_main2;
pub mod simple_mc_main3;
pub mod simple_mc_main4;
pub mod simple_mc_main5;
pub mod stats_main1;
pub mod stats_main2;
pub mod vanilla_main1;
pub mod vanilla_main2;
pub mod vanilla_main3;
pub mod vanilla_main4;
use std::str::FromStr;

#[derive(Debug)]
enum EntryPoints {
    SimpleMcMain1,
    SimpleMcMain2,
    SimpleMcMain3,
    SimpleMcMain4,
    SimpleMcMain5,
    VanillaMain1,
    VanillaMain2,
    VanillaMain3,
    VanillaMain4,
    StatsMain1,
    StatsMain2,
    RandomMain3,
    EquityFXMain,
}

impl FromStr for EntryPoints {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "simple-mc-main1" => Ok(EntryPoints::SimpleMcMain1),
            "simple-mc-main2" => Ok(EntryPoints::SimpleMcMain2),
            "simple-mc-main3" => Ok(EntryPoints::SimpleMcMain3),
            "simple-mc-main4" => Ok(EntryPoints::SimpleMcMain4),
            "simple-mc-main5" => Ok(EntryPoints::SimpleMcMain5),
            "vanilla-main1" => Ok(EntryPoints::VanillaMain1),
            "vanilla-main2" => Ok(EntryPoints::VanillaMain2),
            "vanilla-main3" => Ok(EntryPoints::VanillaMain3),
            "vanilla-main4" => Ok(EntryPoints::VanillaMain4),
            "stats-main1" => Ok(EntryPoints::StatsMain1),
            "stats-main2" => Ok(EntryPoints::StatsMain2),
            "random-main3" => Ok(EntryPoints::RandomMain3),
            "equity-fx-main" => Ok(EntryPoints::EquityFXMain),
            _ => Err("Invalid entry-point."),
        }
    }
}

pub fn main() {
    let mut entry_point = String::new();
    std::io::stdin()
        .read_line(&mut entry_point)
        .expect("Error occurred when reading input string.");
    entry_point = entry_point.trim().to_string();
    match EntryPoints::from_str(&entry_point) {
        Ok(EntryPoints::SimpleMcMain1) => simple_mc_main1::main(),
        Ok(EntryPoints::SimpleMcMain2) => simple_mc_main2::main(),
        Ok(EntryPoints::SimpleMcMain3) => simple_mc_main3::main(),
        Ok(EntryPoints::SimpleMcMain4) => simple_mc_main4::main(),
        Ok(EntryPoints::SimpleMcMain5) => simple_mc_main5::main(),
        Ok(EntryPoints::VanillaMain1) => vanilla_main1::main(),
        Ok(EntryPoints::VanillaMain2) => vanilla_main2::main(),
        Ok(EntryPoints::VanillaMain3) => vanilla_main3::main(),
        Ok(EntryPoints::VanillaMain4) => vanilla_main4::main(),
        Ok(EntryPoints::StatsMain1) => stats_main1::main(),
        Ok(EntryPoints::StatsMain2) => stats_main2::main(),
        Ok(EntryPoints::RandomMain3) => random_main3::main(),
        Ok(EntryPoints::EquityFXMain) => equity_fx_main::main(),
        _ => println!("{}", "wrong implement of pattern matching."),
    }
}
