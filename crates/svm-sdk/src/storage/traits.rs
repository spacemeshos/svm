pub trait Storage {
    fn get32(var_id: u32) -> u32;

    fn get64(var_id: u32) -> u64;

    fn set32(var_id: u32, value: u32);

    fn set64(var_id: u32, value: u64);

    fn store160(var_id: u32, offset: usize);

    fn load160(var_id: u32, offset: usize);
}
