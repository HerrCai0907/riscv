pub enum Exception {
    LoadAccessFault { address: u64 },
    StoreAMOAccessFault { address: u64 },
    UnknownInstructionOpcode { opcode: u32 },
}
