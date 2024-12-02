pub trait ByteStatus {
    /// Add a flag
    fn add(&mut self, bit: u8);

    /// Remove a flag
    fn remove(&mut self, bit: u8);

    /// Check if a flag is set
    fn is_set(&self, bit: u8) -> bool;

    /// Reset the status
    fn reset(&mut self);

    /// Set the status to a specific value
    fn set_bits(&mut self, bits: u8);
}