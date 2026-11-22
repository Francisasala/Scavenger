#![no_main]

use libfuzzer_sys::fuzz_target;
use soroban_sdk::{Address, Env};
use stellar_scavngr_contract::{ScavengerContract, ScavengerContractClient, WasteType};
use std::panic::catch_unwind;

fuzz_target!(|data: &[u8]| {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _admin) = setup(&env);

    // ---- create_incentive(rewarder, waste_type, reward_points, total_budget) ----
    let waste_type_val = data.get(0).copied().unwrap_or(0u8) % 7;
    let waste_type = WasteType::from_u32(waste_type_val as u32).unwrap_or(WasteType::Plastic);

    let reward_points = u64::from_le_bytes([
        data.get(1).copied().unwrap_or(0), data.get(2).copied().unwrap_or(0),
        data.get(3).copied().unwrap_or(0), data.get(4).copied().unwrap_or(0),
        data.get(5).copied().unwrap_or(0), data.get(6).copied().unwrap_or(0),
        data.get(7).copied().unwrap_or(0), data.get(8).copied().unwrap_or(0),
    ]);
    let total_budget = u64::from_le_bytes([
        data.get(9).copied().unwrap_or(0), data.get(10).copied().unwrap_or(0),
        data.get(11).copied().unwrap_or(0), data.get(12).copied().unwrap_or(0),
        data.get(13).copied().unwrap_or(0), data.get(14).copied().unwrap_or(0),
        data.get(15).copied().unwrap_or(0), data.get(16).copied().unwrap_or(0),
    ]);
    let rewarder = Address::generate(&env);

    catch_unwind(std::panic::AssertUnwindSafe(|| {
        client.create_incentive(&rewarder, &waste_type, &reward_points, &total_budget);
    })).ok();

    // ---- claim_incentive_reward(incentive_id, material_id, claimer) ----
    let incentive_id = u64::from_le_bytes([
        data.get(17).copied().unwrap_or(0), data.get(18).copied().unwrap_or(0),
        data.get(19).copied().unwrap_or(0), data.get(20).copied().unwrap_or(0),
        data.get(21).copied().unwrap_or(0), data.get(22).copied().unwrap_or(0),
        data.get(23).copied().unwrap_or(0), data.get(24).copied().unwrap_or(0),
    ]);
    let material_id = u64::from_le_bytes([
        data.get(25).copied().unwrap_or(0), data.get(26).copied().unwrap_or(0),
        data.get(27).copied().unwrap_or(0), data.get(28).copied().unwrap_or(0),
        data.get(29).copied().unwrap_or(0), data.get(30).copied().unwrap_or(0),
        data.get(31).copied().unwrap_or(0), data.get(32).copied().unwrap_or(0),
    ]);
    let claimer = Address::generate(&env);

    catch_unwind(std::panic::AssertUnwindSafe(|| {
        client.claim_incentive_reward(&incentive_id, &material_id, &claimer);
    })).ok();
});

fn setup(env: &Env) -> (ScavengerContractClient, Address) {
    let contract_id = env.register_contract(None, ScavengerContract);
    let client = ScavengerContractClient::new(env, &contract_id);
    let admin = Address::generate(env);
    client.initialize_admin(&admin);
    (client, admin)
}
