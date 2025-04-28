#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol, symbol_short, String, Vec};

// Struct to store verification record of each student
#[contracttype]
#[derive(Clone)]
pub struct Verification {
    pub student: Address,
    pub exam_id: String,
    pub verified: bool,
    pub timestamp: u64,
}

// Key enums for storing records
#[contracttype]
pub enum VerifierKey {
    Record(Address, String), // (Student, ExamID)
    AllRecords,
}

#[contract]
pub struct OnlineExamVerifier;

#[contractimpl]
impl OnlineExamVerifier {
    // Function to verify student for an exam
    pub fn verify_student(env: Env, student: Address, exam_id: String) {
        let timestamp = env.ledger().timestamp();
        let record = Verification {
            student: student.clone(),
            exam_id: exam_id.clone(),
            verified: true,
            timestamp,
        };

        env.storage().instance().set(&VerifierKey::Record(student.clone(), exam_id.clone()), &record);

        let mut all: Vec<(Address, String)> = env.storage().instance().get(&VerifierKey::AllRecords).unwrap_or(Vec::new(&env));
        all.push_back((student, exam_id));
        env.storage().instance().set(&VerifierKey::AllRecords, &all);
    }

    // Function to get a student's verification status for an exam
    pub fn get_verification(env: Env, student: Address, exam_id: String) -> Verification {
        env.storage().instance().get(&VerifierKey::Record(student, exam_id)).unwrap()
    }

    // Function to view all verified records
    pub fn list_all_records(env: Env) -> Vec<(Address, String)> {
        env.storage().instance().get(&VerifierKey::AllRecords).unwrap_or(Vec::new(&env))
    }
}
