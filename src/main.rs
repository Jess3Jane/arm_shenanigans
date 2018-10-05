extern crate unicorn;
extern crate arm_shenanigans;
use arm_shenanigans::{Preprocessor, assemble};
use arm_shenanigans::util::*;
use std::io::prelude::*;
use std::fs::File;
use unicorn::{Cpu, CpuARM};

struct Builder {
    add_code: String,
}

impl Builder {
    fn new() -> Builder {
        let mut add_code = String::new();
        File::open("templates/add.s").unwrap().read_to_string(&mut add_code).unwrap();

        Builder{ add_code }
    }

    fn add(&self, a: &str, b: &str, c: &str, tmp_addr: usize) -> String{
        self.add_code
            .replace("!a", a)
            .replace("!b", b)
            .replace("!c", c)
            .replace("!tmp", &format!("#{}", tmp_addr))
    }
}

fn main() {
    let b = Builder::new();

    let mut code = String::new();
    code.push_str(&vec![
        "movw r0, #0x1234",
        "movt r0, #0x5678",
        "movw r1, #0xabcd",
        "movt r1, #0xef12",
        "\n",
    ].join("\n"));
    code.push_str(&b.add("r0", "r1", "r2", 0x100));

    let code = match assemble(&code) {
        Ok(v) => v,
        Err(e) => {eprintln!("{}", e); return;},
    };

    let mut emu = CpuARM::new(unicorn::Mode::LITTLE_ENDIAN).unwrap();
    emu.mem_map(0, (code.len() / 0x1000 + 1) * 0x1000, unicorn::PROT_ALL);
    emu.mem_write(0, &code);
    emu.emu_start(0, code.len() as u64, 10 * unicorn::SECOND_SCALE, 100);
    print_registers(&emu);
    print_words(&emu.mem_read(0x100, 16).unwrap(), 0x100, 1);
}
