const PAGE_IDX: u32 = 0;

/// Offsets:
/// 32 bytes
const PUB_KEY2_OFFSET: u32 = PUB_KEY_SIZE;
const PUB_KEY3_OFFSET: u32 = PUB_KEY2_OFFSET + PUB_KEY_SIZE;
const LAST_PUB_KEY_OFFSET: u32 = PUB_KEY3_OFFSET + PUB_KEY_SIZE;

/// 8 bytes
const VESTING_START_OFFSET: u32 = LAST_PUB_KEY_OFFSET + PUB_KEY_SIZE;
const LAST_SYNC_LAYER_OFFSET: u32 = VESTING_START_OFFSET + VESTING_START_SIZE;

/// 4 bytes
const BALANCE_OFFSET: u32 = LAST_SYNC_LAYER_OFFSET + LAYER_ID_SIZE;
const VESTED_OFFSET: u32 = BALANCE_OFFSET + BALANCE_SIZE;
const MAX_VESTING_OFFSET: u32 = VESTED_OFFSET + BALANCE_SIZE;

/// 2 bytes
const DAILY_LIMIT_OFFSET: u32 = MAX_VESTING_OFFSET + BALANCE_SIZE;
const VESTING_MONTHS_OFFSET: u32 = DAILY_LIMIT_OFFSET + DAILY_LIMIT_SIZE;

/// Other
const PUBLIC_KEY_FIELD_IDX: u32 = 0;
const IN_FUNC_BUF_ID: u32 = 0;
const ADDRESS_SIZE: u32 = 20;

/// Sizes
/// 32 bytes
const PUB_KEY_SIZE: u32 = 32;

/// 8 bytes
const VESTING_START_SIZE: u32 = 8;
const LAYER_ID_SIZE: u32 = 8;

/// 4 bytes
const BALANCE_SIZE: u32 = 4;

/// 2 bytes
const DAILY_LIMIT_SIZE: u32 = 2;
const VESTING_MONTHS_SIZE: u32 = 2;
