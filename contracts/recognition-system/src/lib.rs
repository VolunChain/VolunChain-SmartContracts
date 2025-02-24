#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, 
    Address, Env, Map, Symbol, Vec,
};

mod datatype;
mod metadata;
mod token;

#[cfg(test)]
mod test;

#[contract]
pub struct RecognitionSystem;

#[contractimpl]
impl RecognitionSystem {
}