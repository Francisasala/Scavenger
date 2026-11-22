#![no_main]

use libfuzzer_sys::fuzz_target;
use soroban_sdk::{Address, Env, Symbol};
use stellar_scavngr_contract::{ParticipantRole, ScavengerContract, ScavengerContractClient};
use std::panic::catch_unwind;

fuzz_target!(|data: &[u8]| {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _admin) = setup(&env);
    let participant = Address::generate(&env);

    // Fuzz participant registration: role, name, latitude, longitude.
    let role_val = data.get(0).copied().unwrap_or(0u8);
    let role = ParticipantRole::from_u32(role_val as u32).unwrap_or(ParticipantRole::Recycler);

    let name_bytes = if data.len() > 1 {
        &data[1..std::cmp::min(data.len(), 65)]
    } else {
        b"fuzz"
    };
    let name = Symbol::new(&env, std::str::from_utf8(name_bytes).unwrap_or("fuzz"));

    let lat = i128::from_le_bytes([
        data.get(0).copied().unwrap_or(0), data.get(1).copied().unwrap_or(0),
        data.get(2).copied().unwrap_or(0), data.get(3).copied().unwrap_or(0),
        data.get(4).copied().unwrap_or(0), data.get(5).copied().unwrap_or(0),
        data.get(6).copied().unwrap_or(0), data.get(7).copied().unwrap_or(0),
        data.get(8).copied().unwrap_or(0), data.get(9).copied().unwrap_or(0),
        data.get(10).copied().unwrap_or(0), data.get(11).copied().unwrap_or(0),
        data.get(12).copied().unwrap_or(0), data.get(13).copied().unwrap_or(0),
        data.get(14).copied().unwrap_or(0), data.get(15).copied().unwrap_or(0),
    ]);
    let lon = i128::from_le_bytes([
        data.get(16).copied().unwrap_or(0), data.get(17).copied().unwrap_or(0),
        data.get(18).copied().unwrap_or(0), data.get(19).copied().unwrap_or(0),
        data.get(20).copied().unwrap_or(0), data.get(21).copied().unwrap_or(0),
        data.get(22).copied().unwrap_or(0), data.get(23).copied().unwrap_or(0),
        data.get(24).copied().unwrap_or(0), data.get(25).copied().unwrap_or(0),
        data.get(26).copied().unwrap_or(0), data.get(27).copied().unwrap_or(0),
        data.get(28).copied().unwrap_or(0), data.get(29).copied().unwrap_or(0),
        data.get(30).copied().unwrap_or(0), data.get(31).copied().unwrap_or(0),
    ]);

    catch_unwind(std::panic::AssertUnwindSafe(|| {
        client.register_participant(&participant, &role, &name, &lat, &lon);
    })).ok();
});

fn setup(env: &Env) -> (ScavengerContractClient, Address) {
    let contract_id = env.register_contract(None, ScavengerContract);
    let client = ScavengerContractClient::new(env, &contract_id);
    let admin = Address::generate(env);
    client.initialize_admin(&admin);
    (client, admin)
}
