#![no_main]

use libfuzzer_sys::fuzz_target;
use soroban_sdk::{Address, Env};
use stellar_scavngr_contract::{ScavengerContract, ScavengerContractClient};
use std::panic::catch_unwind;

fuzz_target!(|data: &[u8]| {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _admin) = setup(&env);

    // ---- reserve_waste(waste_id, reserver, duration) ----
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
    let reserver = Address::generate(&env);
    let duration = u64::from_le_bytes([
        data.get(16).copied().unwrap_or(0), data.get(17).copied().unwrap_or(0),
        data.get(18).copied().unwrap_or(0), data.get(19).copied().unwrap_or(0),
        data.get(20).copied().unwrap_or(0), data.get(21).copied().unwrap_or(0),
        data.get(22).copied().unwrap_or(0), data.get(23).copied().unwrap_or(0),
    ]);

    catch_unwind(std::panic::AssertUnwindSafe(|| {
        client.reserve_waste(&waste_id, &reserver, &duration);
    })).ok();
});

fn setup(env: &Env) -> (ScavengerContractClient, Address) {
    let contract_id = env.register_contract(None, ScavengerContract);
    let client = ScavengerContractClient::new(env, &contract_id);
    let admin = Address::generate(env);
    client.initialize_admin(&admin);
    (client, admin)
}
