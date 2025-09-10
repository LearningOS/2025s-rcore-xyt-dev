use riscv::register::{sscratch, sstatus::{self, Sstatus, SPP}};
/// Trap Context
#[repr(C)]
pub struct TrapContext {
    /// general regs[0..31]
    pub x: [usize; 32],
    /// CSR sstatus      
    pub sstatus: Sstatus,
    /// CSR sepc
    pub sepc: usize,
}

impl TrapContext {
    /// set stack pointer to x_2 reg (sp)
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }
    /// init app context
    pub fn app_init_context(entry: usize, sp: usize) -> Self {
        let mut sstatus = sstatus::read(); // CSR sstatus
        // print_sstatus_bits();
        // let mut sscratch = sscratch::read();
        // println!("sscratch: {}", sscratch);
        sstatus.set_spp(SPP::User); //previous privilege mode: user mode
        let mut cx = Self {
            x: [0; 32],
            sstatus,
            sepc: entry, // entry point of app
        };
        cx.set_sp(sp); // app's user stack pointer
        cx // return initial Trap Context of app
    }
}

// 一个函数来按位打印 sstatus
pub fn print_sstatus_bits() {
    let sstatus = sstatus::read(); // 读取当前 sstatus 值（S-mode 下有效）
    let raw_value = sstatus.bits(); // 获取原始 u64 值（或 usize，视架构）

    // 按位分解并打印常见字段（你可以添加更多）
    println!("- SD (bit 63): {}", (raw_value >> 63) & 1);
    println!("- MXR (bit 33): {}", (raw_value >> 33) & 1);
    println!("- SUM (bit 32): {}", (raw_value >> 32) & 1);
    println!("- XS (bits 19): {}", (raw_value >> 19) & 1); // 简化，实际可能是多位
    println!("- FS (bit 18): {}", (raw_value >> 18) & 1);
    println!("- SPP (bit 8): {} (Previous Privilege: {} mode)", 
             (raw_value >> 8) & 1, 
             if (raw_value >> 8) & 1 == 1 { "Supervisor" } else { "User" });
    println!("- SPIE (bit 5): {} (Previous Interrupt Enable)", (raw_value >> 5) & 1);
    println!("- SIE (bit 1): {} (Supervisor Interrupt Enable)", (raw_value >> 1) & 1);

    // 如果想打印所有 64 位，可以循环输出
    println!("All bits (from MSB to LSB):");
    for i in (0..64).rev() { // 从高位到低位打印
        let bit = (raw_value >> i) & 1;
        print!("{} ", bit);
        if (63 - i) % 8 == 7 { println!(""); } // 每 8 位换行，便于阅读
    }
}