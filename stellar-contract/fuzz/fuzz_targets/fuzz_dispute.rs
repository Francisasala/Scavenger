#![no_main]

use libfuzzer_sys::fuzz_target;
use soroban_sdk::{Address, Env, String as SorobanString};
use stellar_scavngr_contract::{ScavengerContract, ScavengerContractClient};
use std::panic::catch_unwind;

fuzz_target!(|data: &[u8]| {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _admin) = setup(&env);

    // ---- create_dispute(disputer, waste_id, reason) ----
    let disputer = Address::generate(&env);

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
    // Keep the reason well under the 500-char contract cap.
    let reason_bytes = if data.len() > 16 {
        &data[16..std::cmp::min(data.len(), 80)]
    } else {
        b"fuzz"
    };
    let reason = SorobanString::from_str(&env, std::str::from_utf8(reason_bytes).unwrap_or("fuzz"));

    catch_unwind(std::panic::AssertUnwindSafe(|| {
        client.create_dispute(&disputer, &waste_id, &reason);
    })).ok();
});

fn setup(env: &Env) -> (ScavengerContractClient, Address) {
    let contract_id = env.register_contract(None, ScavengerContract);
    let client = ScavengerContractClient::new(env, &contract_id);
    let admin = Address::generate(env);
    client.initialize_admin(&admin);
    (client, admin)
}
