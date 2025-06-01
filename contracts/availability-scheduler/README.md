# Availability Scheduler Smart Contract

A Soroban smart contract for managing volunteer availability schedules in the VolunChain ecosystem.

## Overview

The Availability Scheduler contract allows volunteers to set and manage their availability time slots for different days of the week. It provides a flexible system for organizations to coordinate volunteer schedules and match them with appropriate tasks.

## Features

- Set availability for specific days of the week
- Manage multiple time slots per day
- Store and retrieve availability data for individual volunteers
- Admin controls for contract management
- Event emission for availability updates

## Contract Structure

### Storage

The contract uses persistent storage to maintain:
- Admin address
- Volunteer availability data (organized by volunteer address and day)

### Key Functions

#### Admin Functions
- `write_admin`: Sets the contract admin
- `read_admin`: Retrieves the current admin address

#### Availability Management
- `write_availability`: Sets availability slots for a volunteer on a specific day
- `read_availability`: Retrieves availability slots for a volunteer on a specific day
- `read_all_availability`: Retrieves all availability data for a volunteer

### Events

The contract emits events for important state changes:
- Availability updates
- Admin changes

## Usage

### Setting Availability

```rust
// Set availability for a volunteer
write_availability(
    &env,
    &volunteer_address,
    day, // 0-6 representing days of the week
    &time_slots // Vector of (start_hour, end_hour) tuples
);
```

### Reading Availability

```rust
// Get availability for a specific day
let slots = read_availability(&env, &volunteer_address, day);

// Get all availability data
let all_slots = read_all_availability(&env, &volunteer_address);
```

## Time Slot Format

- Time slots are represented as tuples of (start_hour, end_hour)
- Hours are in 24-hour format (1-24)
- Multiple slots can be set for each day
- Slots must not overlap

## Security

- Admin functions require proper authorization
- Input validation for time slots
- Overlap checking for time slots
- Proper access control for volunteer data

## Testing

The contract includes comprehensive test coverage:
- Basic availability setting and retrieval
- Multiple volunteers handling
- Time slot validation
- Admin operations
- Event emission verification

## Dependencies

- soroban-sdk: ^22.0.0

## License

This project is licensed under the MIT License - see the LICENSE file for details. 