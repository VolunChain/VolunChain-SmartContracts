use soroban_sdk::{Address, Env, Map, Symbol, Vec};

fn get_admin_key(env: &Env) -> Symbol {
    Symbol::new(env, "admin")
}

fn get_availability_key(env: &Env, volunteer: &Address, _day: u32) -> (Symbol, Address) {
    (Symbol::new(env, "availability"), volunteer.clone())
}

pub fn write_admin(env: &Env, admin: &Address) {
    env.storage().persistent().set(&get_admin_key(env), admin);
}

#[allow(dead_code)]
pub fn read_admin(env: &Env) -> Address {
    env.storage().persistent().get::<_, Address>(&get_admin_key(env))
        .unwrap_or_else(|| panic!("Admin not set"))
}

pub fn write_availability(env: &Env, volunteer: &Address, day: u32, time_slots: &Vec<(u32, u32)>) {
    let (key, volunteer_key) = get_availability_key(env, volunteer, day);
    // Get existing map or create new one
    let mut availability_map: Map<u32, Vec<(u32, u32)>> = env.storage().persistent()
        .get(&(key.clone(), volunteer_key.clone()))
        .unwrap_or_else(|| Map::new(env));
    
    // Update the map with new time slots
    availability_map.set(day, time_slots.clone());
    
    // Store the updated map
    env.storage().persistent().set(&(key, volunteer_key), &availability_map);
}

pub fn read_availability(env: &Env, volunteer: &Address, day: u32) -> Vec<(u32, u32)> {
    let (key, volunteer_key) = get_availability_key(env, volunteer, day);
    env.storage().persistent()
        .get::<_, Map<u32, Vec<(u32, u32)>>>(&(key, volunteer_key))
        .and_then(|map| map.get(day))
        .unwrap_or_else(|| Vec::new(env))
}

pub fn read_all_availability(env: &Env, volunteer: &Address) -> Map<u32, Vec<(u32, u32)>> {
    let (key, volunteer_key) = get_availability_key(env, volunteer, 0); // day doesn't matter here
    env.storage().persistent()
        .get::<_, Map<u32, Vec<(u32, u32)>>>(&(key, volunteer_key))
        .unwrap_or_else(|| Map::new(env))
} 