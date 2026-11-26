//! SwissTable SIMD 탐사 개념 시연 (SIMD 없이 동작 원리만 모사)

const GROUP_SIZE: usize = 16;
const EMPTY: u8 = 0xFF;
const DELETED: u8 = 0x80;

fn match_byte(ctrl: &[u8; GROUP_SIZE], needle: u8) -> u16 {
    let mut mask = 0;
    for (i, &byte) in ctrl.iter().enumerate() {
        if byte == needle {
            mask |= 1 << i;
        }
    }

    mask
}

fn match_empty(ctrl: &[u8; GROUP_SIZE]) -> u16 {
    // EMPTY(0xFF)는 최상위 비트 = 1 이면서 DELETED(0x80)가 아닌 것
    // 실제로는 bit trick: byte & 0x80 이고 byte != 0x80
    let mut mask = 0;
    for (i, &byte) in ctrl.iter().enumerate() {
        if byte == EMPTY {
            mask |= 1 << i;
        }
    }

    mask
}

fn probe_group(ctrl: &[u8; GROUP_SIZE], h2: u8) -> Vec<usize> {
    let mask = match_byte(ctrl, h2);
    // 비트마스크에서 일치하는 인덱스 추출
    let mut candidate = Vec::new();
    let mut remaining = mask;
    while remaining != 0 {
        let idx = remaining.trailing_zeros() as usize;
        candidate.push(idx);
        // 최하위 비트 제거
        remaining &= remaining - 1;
    }

    candidate
}

fn main() {
    // 16슬롯 ctrl 배열 예시
    let mut ctrl = [EMPTY; GROUP_SIZE];
    // h2=0x42인 항목
    ctrl[0] = 0x42;
    // h2=0x17인 항목
    ctrl[3] = 0x17;
    // h2=0x42인 항목 (충돌)
    ctrl[7] = 0x42;
    ctrl[11] = DELETED;

    println!("ctrl 배열:");
    for (i, &c) in ctrl.iter().enumerate() {
        let status = if c == EMPTY {
            "EMPTY  ".to_string()
        } else if c == DELETED {
            "DELETED".to_string()
        } else {
            format!("h2=0x{:02x}", c)
        };

        println!("  ctrl[{:2}] = {}", i, status);
    }

    let search_h2 = 0x42u8;
    let candidates = probe_group(&ctrl, search_h2);
    println!("\nh2=0x{:02x} 탐사 결과: {:?}", search_h2, candidates);

    let empty_mask = match_empty(&ctrl);
    println!("EMPTY 슬롯 마스크: {:016b}", empty_mask);
}
