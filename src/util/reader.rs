
/* very common functions, so inline */

/* host endian independent multi-byte integer reading */

#[inline(always)]
pub fn get_16bitBE(data: &[u8]) -> i16 {
    return ((data[0] as i16) << 8) | (data[1] as i16);
}

#[inline(always)]
pub fn get_16bitLE(data: &[u8]) -> i16 {
    return ((data[1] as i16) << 8) | (data[0] as i16);
}

#[inline(always)]
pub fn get_32bitBE(data: &[u8]) -> i32 {
    return ((data[0] as i32) << 24) | ((data[1] as i32) << 16) | ((data[2] as i32) << 8) | (data[3] as i32);
}

#[inline(always)]
pub fn get_32bitLE(data: &[u8]) -> i32 {
    return ((data[3] as i32) << 24) | ((data[2] as i32) << 16) | ((data[1] as i32) << 8) | (data[0] as i32);
}

#[inline(always)]
pub fn get_64bitBE(data: &[u8]) -> i64 {
    return ((data[0] as i64) << 56) | ((data[1] as i64) << 48) | ((data[2] as i64) << 40) | ((data[3] as i64) << 32) | ((data[4] as i64) << 24) | ((data[5] as i64) << 16) | ((data[6] as i64) << 8) | (data[7] as i64);
}

#[inline(always)]
pub fn get_64bitLE(data: &[u8]) -> i64 {
    return ((data[7] as i64) << 56) | ((data[6] as i64) << 48) | ((data[5] as i64) << 40) | ((data[4] as i64) << 32) | ((data[3] as i64) << 24) | ((data[2] as i64) << 16) | ((data[1] as i64) << 8) | (data[0] as i64);
}

/* alias of the above */

#[inline(always)]
pub fn get_s8(data: &[u8]) -> i8 {
    return data[0] as i8;
}

#[inline(always)]
pub fn get_u8(data: &[u8]) -> u8 {
    return data[0];
}

#[inline(always)]
pub fn get_s16le(data: &[u8]) -> i16 {
    return get_16bitLE(data);
}

#[inline(always)]
pub fn get_s16be(data: &[u8]) -> i16 {
    return get_16bitBE(data);
}

#[inline(always)]
pub fn get_u16le(data: &[u8]) -> u16 {
    return get_16bitLE(data) as u16;
}

#[inline(always)]
pub fn get_u16be(data: &[u8]) -> u16 {
    return get_16bitBE(data) as u16;
}

#[inline(always)]
pub fn get_s32le(data: &[u8]) -> i32 {
    return get_32bitLE(data);
}

#[inline(always)]
pub fn get_s32be(data: &[u8]) -> i32 {
    return get_32bitBE(data);
}

#[inline(always)]
pub fn get_u32le(data: &[u8]) -> u32 {
    return get_32bitLE(data) as u32;
}

#[inline(always)]
pub fn get_u32be(data: &[u8]) -> u32 {
    return get_32bitBE(data) as u32;
}

#[inline(always)]
pub fn get_s64le(data: &[u8]) -> i64 {
    return get_64bitLE(data);
}

#[inline(always)]
pub fn get_s64be(data: &[u8]) -> i64 {
    return get_64bitBE(data);
}

#[inline(always)]
pub fn get_u64le(data: &[u8]) -> u64 {
    return get_64bitLE(data) as u64;
}

#[inline(always)]
pub fn get_u64be(data: &[u8]) -> u64 {
    return get_64bitBE(data) as u64;
}

#[inline(always)]
pub fn get_f32le(data: &[u8]) -> f32 {
    return f32::from_bits(get_u32le(data));
}

#[inline(always)]
pub fn get_f32be(data: &[u8]) -> f32 {
    return f32::from_bits(get_u32be(data));
}

#[inline(always)]
pub fn get_f64le(data: &[u8]) -> f64 {
    return f64::from_bits(get_u64le(data));
}

#[inline(always)]
pub fn get_f64be(data: &[u8]) -> f64 {
    return f64::from_bits(get_u64be(data));
}


/* signed nibbles come up a lot in decoders */
pub const NIBBLE_TO_INT: [i32;16] = [0,1,2,3,4,5,6,7,-8,-7,-6,-5,-4,-3,-2,-1];

#[inline(always)]
pub fn get_nibble_signed(n: u8, upper: i32) -> i32 {
    /*return ((n&0x70)-(n&0x80))>>4;*/
    return NIBBLE_TO_INT[(n >> ( if upper != 0 {4} else {0})) as usize & 0x0f];
}

pub fn get_high_nibble_signed(n: u8) -> i32 {
    /*return ((n&0x70)-(n&0x80))>>4;*/
    return NIBBLE_TO_INT[n as usize>>4];
}

pub fn get_low_nibble_signed(n: u8) -> i32 {
    /*return (n&7)-(n&8);*/
    return NIBBLE_TO_INT[n as usize&0xf];
}