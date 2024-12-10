pub mod address;

pub struct MemoryInterfaceImpl;

impl super::interface::MemoryInterface for MemoryInterfaceImpl {
    type VAddr = address::vaddr::VAddr;
    type PAddr = address::paddr::PAddr;
}