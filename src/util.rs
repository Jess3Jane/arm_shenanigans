use unicorn::{Cpu, CpuARM};
use unicorn;

fn print_columns(mut columns: Vec<Box<Vec<String>>>, col_seperator: &str) {
    let mut rows = 0;
    for v in &columns {
        rows = std::cmp::max(v.len(), rows);
    }

    for v in &mut columns {
        let mut width = v[0].len();

        let mut s = String::new();
        for _ in 0..width { s.push(' '); }
        for _ in 0..(rows - v.len()) {
            v.push(String::from(s.clone()));
        }
    }

    let mut iters = columns.into_iter().map(|v| v.into_iter())
        .collect::<Vec<std::vec::IntoIter<String>>>();
    'outer: loop {
        let mut row = String::new();
        for iter in &mut iters {
            row.push_str(&match iter.next() {
                Some(v) => v,
                None => {break 'outer;},
            });
            row.push_str(col_seperator);
        }
        println!("{}", row);
    }
}

pub fn print_registers(emu: &CpuARM) {
    let mut columns = Vec::new();

    let mut rows = Vec::new();
    rows.push(format!("{:<4}{:08x}", "R0", emu.reg_read(unicorn::arm_const::RegisterARM::R0).unwrap()));
    rows.push(format!("{:<4}{:08x}", "R1", emu.reg_read(unicorn::arm_const::RegisterARM::R1).unwrap()));
    rows.push(format!("{:<4}{:08x}", "R2", emu.reg_read(unicorn::arm_const::RegisterARM::R2).unwrap()));
    rows.push(format!("{:<4}{:08x}", "R3", emu.reg_read(unicorn::arm_const::RegisterARM::R3).unwrap()));
    rows.push(format!("{:<4}{:08x}", "R4", emu.reg_read(unicorn::arm_const::RegisterARM::R4).unwrap()));
    rows.push(format!("{:<4}{:08x}", "R5", emu.reg_read(unicorn::arm_const::RegisterARM::R5).unwrap()));
    rows.push(format!("{:<4}{:08x}", "R6", emu.reg_read(unicorn::arm_const::RegisterARM::R6).unwrap()));
    rows.push(format!("{:<4}{:08x}", "R7", emu.reg_read(unicorn::arm_const::RegisterARM::R7).unwrap()));
    rows.push(format!("{:<4}{:08x}", "R8", emu.reg_read(unicorn::arm_const::RegisterARM::R8).unwrap()));
    rows.push(format!("{:<4}{:08x}", "R9", emu.reg_read(unicorn::arm_const::RegisterARM::SB).unwrap()));
    rows.push(format!("{:<4}{:08x}", "R10", emu.reg_read(unicorn::arm_const::RegisterARM::SL).unwrap()));
    rows.push(format!("{:<4}{:08x}", "R11", emu.reg_read(unicorn::arm_const::RegisterARM::FP).unwrap()));
    rows.push(format!("{:<4}{:08x}", "R12", emu.reg_read(unicorn::arm_const::RegisterARM::IP).unwrap()));
    columns.push(Box::new(rows));

    let mut rows = Vec::new();
    rows.push(format!("{:<10}{:08x}", "APSR", 
        emu.reg_read(unicorn::arm_const::RegisterARM::APSR).unwrap()));
    rows.push(format!("{:<10}{:08x}", "APSR_NZCV", 
        emu.reg_read(unicorn::arm_const::RegisterARM::APSR_NZCV).unwrap()));
    rows.push(format!("{:<10}{:08x}", "CPSR", 
        emu.reg_read(unicorn::arm_const::RegisterARM::CPSR).unwrap()));
    rows.push(format!("{:<10}{:08x}", "LR", 
        emu.reg_read(unicorn::arm_const::RegisterARM::LR).unwrap()));
    rows.push(format!("{:<10}{:08x}", "PC", 
        emu.reg_read(unicorn::arm_const::RegisterARM::PC).unwrap()));
    rows.push(format!("{:<10}{:08x}", "SP", 
        emu.reg_read(unicorn::arm_const::RegisterARM::SP).unwrap()));
    columns.push(Box::new(rows));

    print_columns(columns, "    ");
}

pub fn print_words(data: &[u8], base_addr: u64, width: u64) {
    let mut columns = Vec::new();
    
    let mut addr_col = Vec::new();
    addr_col.push(format!("{:<10}", "Address"));

    let mut cols = Vec::new();
    for i in 0..width*4 { cols.push(Box::new(vec![format!("{:>8}", format!("{:02x}", i*4))])); }

    let mut col = 0;
    let mut row = 0;
    for b in data.chunks(4) {
        if col == 0 {
            addr_col.push(format!("{:<10}", format!("{:08x}", base_addr + (width*4*row))));
        }

        cols[col].push(format!("{:08x}", b.iter().fold(0, |cur, v| (*v as u32) << 24 | cur >> 8)));

        row += 1;
        col += 1;
        if col == cols.len() { col = 0; }
    }

    columns.push(Box::new(addr_col));
    for col in cols { columns.push(col); }
    print_columns(columns, "  ");
}

pub fn print_half_words(data: &[u8], base_addr: u64, width: u64) {
    let mut columns = Vec::new();
    
    let mut addr_col = Vec::new();
    addr_col.push(format!("{:<10}", "Address"));

    let mut cols = Vec::new();
    for i in 0..width*8 { cols.push(Box::new(vec![format!("{:>4}", format!("{:02x}", i*2))])); }

    let mut col = 0;
    let mut row = 0;
    for b in data.chunks(2) {
        if col == 0 {
            addr_col.push(format!("{:<10}", format!("{:08x}", base_addr + (width*4*row))));
        }

        cols[col].push(format!("{:04x}", b.iter().fold(0, |cur, v| (*v as u32) << 8 | cur >> 8)));

        row += 1;
        col += 1;
        if col == cols.len() { col = 0; }
    }

    columns.push(Box::new(addr_col));
    for col in cols { columns.push(col); }
    print_columns(columns, "  ");
}

pub fn print_bytes(data: &[u8], base_addr: u64, width: u64) {
    let mut columns = Vec::new();
    
    let mut addr_col = Vec::new();
    addr_col.push(format!("{:<10}", "Address"));

    let mut cols = Vec::new();
    for i in 0..width*16 { cols.push(Box::new(vec![format!("{:>2}", format!("{:02x}", i))])); }

    let mut col = 0;
    let mut row = 0;
    for b in data {
        if col == 0 {
            addr_col.push(format!("{:<10}", format!("{:08x}", base_addr + (width*4*row))));
        }

        cols[col].push(format!("{:02x}", b));

        row += 1;
        col += 1;
        if col == cols.len() { col = 0; }
    }

    columns.push(Box::new(addr_col));
    for col in cols { columns.push(col); }
    print_columns(columns, "  ");
}
