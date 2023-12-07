#[derive(Debug, PartialEq)]
pub enum Exception {
    LoadAccessFault { address: u64 },
    StoreAMOAccessFault { address: u64 },
    EnvironmentCall,
    Breakpoint,
    InvalidInstruction,
}
