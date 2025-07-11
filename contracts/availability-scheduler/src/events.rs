use soroban_sdk::{Address, Env, Symbol, Vec};

pub fn emit_availability_updated(
    env: &Env,
    volunteer: &Address,
    day: u32,
    time_slots: &Vec<(u32, u32)>,
) {
    let availability_updated = Symbol::new(env, "availability_updated");
    let volunteer_sym = Symbol::new(env, "volunteer");
    let day_sym = Symbol::new(env, "day");
    let time_slots_sym = Symbol::new(env, "time_slots");

    let topics = (availability_updated, volunteer_sym, volunteer.clone());
    let data = (day_sym, day, time_slots_sym, time_slots.clone());
    env.events().publish(topics, data);
} 