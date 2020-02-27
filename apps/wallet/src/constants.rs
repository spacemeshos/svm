/// Offsets:
/// 32 bytes
const PUB_KEY2_OFFSET: u32 = PUB_KEY_SIZE;
const PUB_KEY3_OFFSET: u32 = PUB_KEY2_OFFSET + PUB_KEY_SIZE;
const IS_MULTISIG_OFFSET: u32 = PUB_KEY3_OFFSET + PUB_KEY_SIZE;
const PENDING_PUB_KEY_OFFSET: u32 = PUB_KEY3_OFFSET + PUB_KEY_SIZE;

/// 8 bytes
const FIRST_LAYER_OFFSET: u32 = 0;
const LAST_RUN_LAYER_OFFSET: u32 = 0;

/// 4 bytes
const LIQUIDATED_OFFSET: u32 = 0;
const UNLIQUIDATED_OFFSET: u32 = 0;

/// 2 bytes
const LAYER_LIQ_OFFSET: u32 = 0;
const DAILY_PULL_LIMIT_OFFSET: u32 = 0;

/// Other
const ADDRESS_SIZE: u32 = 20;
const PUB_KEY_SIZE: u32 = 32;

const FUNC_BUF_ID: u32 = 0;
const PAGE_IDX: u32 = 0;

/// HostCtx fields
const LAYER_ID_FIELD: u32 = 0;
const LAYER_TIME_FIELD: u32 = 0;
const PUBLIC_KEY_FIELD: u32 = 0;
