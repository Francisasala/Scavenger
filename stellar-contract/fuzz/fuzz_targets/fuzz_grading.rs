#![no_main]

use libfuzzer_sys::fuzz_target;
use soroban_sdk::{Address, Env};
use stellar_scavngr_contract::{ScavengerContract, ScavengerContractClient, WasteGrade};
use std::panic::catch_unwind;

fuzz_target!(|data: &[u8]| {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _admin) = setup(&env);

    // ---- set_waste_grade(waste_id, grade, grader) ----
    let waste_id = u128::from_le_bytes([
        data.get(0).copied().unwrap_or(0), data.get(1).copied().unwrap_or(0),
        data.get(2).copied().unwrap_or(0), data.get(3).copied().unwrap_or(0),
        data.get(4).copied().unwrap_or(0), data.get(5).copied().unwrap_or(0),
        data.get(6).copied().unwrap_or(0), data.get(7).copied().unwrap_or(0),
        data.get(8).copied().unwrap_or(0), data.get(9).copied().unwrap_or(0),
        data.get(10).copied().unwrap_or(0), data.get(11).copied().unwrap_or(0),
        data.get(12).copied().unwrap_or(0), data.get(13).copied().unwrap_or(0),
        data.get(14).copied().unwrap_or(0), data.get(15).copied().unwrap_or(0),
    ]);
    let grade_val = data.get(16).copied().unwrap_or(0u8) % 4;
    let grade = WasteGrade::from_u32(grade_val as u32).unwrap_or(WasteGrade::C);
    let grader = Address::generate(&env);

    catch_unwind(std::panic::AssertUnwindSafe(|| {
        client.set_waste_grade(&waste_id, &grade, &grader);
    })).ok();
});

fn setup(env: &Env) -> (ScavengerContractClient, Address) {
    let contract_id = env.register_contract(None, ScavengerContract);
    let client = ScavengerContractClient::new(env, &contract_id);
    let admin = Address::generate(env);
    client.initialize_admin(&admin);
    (client, admin)
}
