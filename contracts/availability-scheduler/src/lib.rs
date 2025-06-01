#![no_std]

use soroban_sdk::{contractimpl, Address, Env, Map, Vec, contract};
use crate::errors::Error;

mod storage;
mod errors;
mod events;
mod test;

#[contract]
pub struct AvailabilityScheduler;

#[contractimpl]
impl AvailabilityScheduler {
    pub fn initialize(env: &Env, admin: Address) {
        admin.require_auth();
        storage::write_admin(env, &admin);
    }

    pub fn set_availability(
        env: &Env,
        volunteer: Address,
        day: u32,
        time_slots: Vec<(u32, u32)>,
    ) -> Result<(), Error> {
        // Verify the volunteer is setting their own availability
        volunteer.require_auth();

        // Validate day (0-6 for Monday-Sunday)
        if day > 6 {
            return Err(Error::InvalidDay);
        }

        // Validate time slots are not empty
        if time_slots.is_empty() {
            return Err(Error::InvalidTimeRange);
        }

        // Validate time slots
        for (start, end) in time_slots.iter() {
            // Validate hour range (1-24)
            if start < 1 || start > 24 || end < 1 || end > 24 {
                return Err(Error::InvalidTimeRange);
            }
            
            if start >= end {
                return Err(Error::InvalidTimeRange);
            }
        }

        // Check for overlapping slots
        for i in 0..time_slots.len() {
            for j in (i + 1)..time_slots.len() {
                let (start1, end1) = time_slots.get_unchecked(i);
                let (start2, end2) = time_slots.get_unchecked(j);
                if !(end1 <= start2 || end2 <= start1) {
                    return Err(Error::OverlappingTimeSlots);
                }
            }
        }

        // Store the availability
        storage::write_availability(env, &volunteer, day, &time_slots);
        
        // Emit event
        events::emit_availability_updated(env, &volunteer, day, &time_slots);

        Ok(())
    }

    pub fn get_availability(env: &Env, volunteer: Address, day: u32) -> Vec<(u32, u32)> {
        storage::read_availability(env, &volunteer, day)
    }

    pub fn get_all_availability(env: &Env, volunteer: Address) -> Map<u32, Vec<(u32, u32)>> {
        storage::read_all_availability(env, &volunteer)
    }
} 