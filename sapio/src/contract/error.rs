// Copyright Judica, Inc 2021
//
// This Source Code Form is subject to the terms of the Mozilla Public
//  License, v. 2.0. If a copy of the MPL was not distributed with this
//  file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! error types that can be returned from Sapio.
//! Where possible, concrete error types are wrapped, but in order to handle
//! errors created by the user we allow boxing an error trait.
use crate::contract::object::ObjectError;
use sapio_base::effects::EffectDBError;
use sapio_base::effects::ValidFragmentError;
use sapio_ctv_emulator_trait::EmulatorError;
use std::collections::LinkedList;
use std::error::Error;
use std::fmt;
/// Sapio's core error type.
#[derive(Debug)]
pub enum CompilationError {
    /// Unspecified Error -- but we should stop compiling
    TerminateCompilation,
    /// Fee Specification Error
    MinFeerateError,
    /// Error when ContextPath has already been used.
    ContexPathAlreadyDerived,
    /// Error when ContextPath attempted
    InvalidPathName,
    /// Other Error for Fragment Format
    PathFragmentError(ValidFragmentError),
    /// Error when a `ThenFunc` returns no Templates.
    MissingTemplates,
    /// Error if a Policy is empty
    EmptyPolicy,
    /// Error if a contract does not have sufficient funds available
    OutOfFunds,
    /// Error if a CheckSequenceVerify clause is incompatible with the sequence already set.
    /// E.g., blocks and time
    IncompatibleSequence,
    /// Error if a CheckLockTime clause is incompatible with the locktime already set.
    /// E.g., blocks and time
    IncompatibleLockTime,
    /// Error if a sequence at index j >= inputs.len() is attempted to be set
    NoSuchSequence,
    /// Error if parsing an Amount failed
    ParseAmountError(bitcoin::util::amount::ParseAmountError),
    /// Error from the Policy Compiler
    Miniscript(miniscript::policy::compiler::CompilerError),
    /// Error from the miniscript system
    MiniscriptE(miniscript::Error),
    /// Error with a Timelock
    TimeLockError(sapio_base::timelocks::LockTimeError),
    /// Error creating an object,
    CompiledObjectError(ObjectError),
    /// Failure in conditional compilation logic
    ConditionalCompilationFailed(LinkedList<String>),
    /// Error fromt the Effects system
    EffectDBError(EffectDBError),
    /// Unknown Error type -- either from a user or from some unhandled dependency
    Custom(Box<dyn std::error::Error>),
}

impl From<ValidFragmentError> for CompilationError {
    fn from(e: ValidFragmentError) -> CompilationError {
        CompilationError::PathFragmentError(e)
    }
}
impl From<EffectDBError> for CompilationError {
    fn from(e: EffectDBError) -> CompilationError {
        CompilationError::EffectDBError(e)
    }
}

impl From<std::convert::Infallible> for CompilationError {
    fn from(_s: std::convert::Infallible) -> CompilationError {
        unimplemented!("Impossible, Just to make Type System Happy...");
    }
}

impl CompilationError {
    /// Create a custom compilation error instance
    pub fn custom<E: std::error::Error + 'static>(e: E) -> Self {
        CompilationError::Custom(Box::new(e))
    }
}

impl From<bitcoin::util::amount::ParseAmountError> for CompilationError {
    fn from(b: bitcoin::util::amount::ParseAmountError) -> Self {
        CompilationError::ParseAmountError(b)
    }
}

impl From<sapio_base::timelocks::LockTimeError> for CompilationError {
    fn from(b: sapio_base::timelocks::LockTimeError) -> Self {
        CompilationError::TimeLockError(b)
    }
}
impl From<miniscript::policy::compiler::CompilerError> for CompilationError {
    fn from(v: miniscript::policy::compiler::CompilerError) -> Self {
        CompilationError::Miniscript(v)
    }
}
impl From<miniscript::Error> for CompilationError {
    fn from(v: miniscript::Error) -> Self {
        CompilationError::MiniscriptE(v)
    }
}
impl From<ObjectError> for CompilationError {
    fn from(e: ObjectError) -> Self {
        CompilationError::CompiledObjectError(e)
    }
}

impl fmt::Display for CompilationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            // Unspecified Error -- but we should stop compiling
            CompilationError::TerminateCompilation => f.write_str("Compile Error: Unspecified!"),
            // Fee Specification Error
            CompilationError::MinFeerateError => f.write_str("Compile Error: Minimum Fee Rate!"),
            // Error when ContextPath has already been used.
            CompilationError::ContexPathAlreadyDerived => f.write_str("Compile Error: ContextPath has already been used!"),
            // Error when ContextPath attempted
            CompilationError::InvalidPathName => f.write_str("Compile Error: InvalidPathName!"),
            // Other Error for Fragment Format
            CompilationError::PathFragmentError(e) => write!(f, "Compile Error; Path Fragment: {:?}", e),
            // Error when a `ThenFunc` returns no Templates.
            CompilationError::MissingTemplates => f.write_str("Compile Error: `ThenFunc` returns no Templates!"),
            // Error if a Policy is empty
            CompilationError::EmptyPolicy => f.write_str("Compile Error: Policy is empty!"),
            // Error if a contract does not have sufficient funds available
            CompilationError::OutOfFunds => f.write_str("Compile Error: Contract does not have sufficient funds available!"),
            // Error if a CheckSequenceVerify clause is incompatible with the sequence already set.
            // E.g., blocks and time
            CompilationError::IncompatibleSequence => f.write_str("Compile Error: CheckSequenceVerify clause is incompatible with the sequence already set!"),
            // Error if a CheckLockTime clause is incompatible with the locktime already set.
            // E.g., blocks and time
            CompilationError::IncompatibleLockTime => f.write_str("Compile Error: CheckLockTime clause is incompatible with the locktime already set!"),
            // Error if a sequence at index j >= inputs.len() is attempted to be set
            CompilationError::NoSuchSequence => f.write_str("Compile Error: No such sequence!"),
            // Error if parsing an Amount failed
            CompilationError::ParseAmountError(e) => write!(f, "Compile Error; Parse Amount: {:?}", e),
            // Error from the Policy Compiler
            CompilationError::Miniscript(e) => write!(f, "Compile Error; Miniscript Policy: {:?}", e),
            // Error from the miniscript system
            CompilationError::MiniscriptE(e) =>  write!(f, "Compile Error; Miniscript: {:?}", e),
            // Error with a Timelock
            CompilationError::TimeLockError(e) => write!(f, "Compile Error; Time Lock: {:?}", e),
            // Error creating an object,
            CompilationError::CompiledObjectError(e) => write!(f, "Compile Error; Object: {:?}", e),
            // Failure in conditional compilation logic
            CompilationError::ConditionalCompilationFailed(_ls) => f.write_str("Compilation Error: Failure in conditional compilation logic!"),
            // Error from the Effects system
            CompilationError::EffectDBError(e) => write!(f, "Compile Error; Effect DB: {:?}", e),
            // Unknown Error type -- either from a user or from some unhandled dependency
            CompilationError::Custom(e) => write!(f, "Compile Error: {:?}", e)
        }
    }
}

impl Error for CompilationError {}

impl From<EmulatorError> for CompilationError {
    fn from(e: EmulatorError) -> Self {
        CompilationError::Custom(Box::new(e))
    }
}
