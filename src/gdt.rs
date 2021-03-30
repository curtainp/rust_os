use lazy_static::lazy_static;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

// this value can change between 0-6
pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;
pub const PAGE_SIZE: usize = 4096;
pub const PAGE_CNT: usize = 5;

pub fn init() {
    use x86_64::instructions::segmentation::set_cs;
    use x86_64::instructions::tables::load_tss;

    GDT.0.load();
    unsafe {
        set_cs(GDT.1.code_segment);
        load_tss(GDT.1.tss_segment);
    }
}

// create a custom TSS
lazy_static! {
    static ref TSS: TaskStateSegment = {
      let mut tss = TaskStateSegment::new();
      tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
        const STACK_SIZE: usize = PAGE_CNT * PAGE_SIZE;
        // TODO, this way is ugly
        static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

        let stack_start = VirtAddr::from_ptr(unsafe{&STACK});
        let stack_end = stack_start + STACK_SIZE;
        //return top address, cause of stack from top to bottom
        stack_end
      };
      tss
    };
}

// create a custom GDT
lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_segment = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_segment = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (
            gdt,
            Selectors {
                code_segment,
                tss_segment,
            },
        )
    };
}

struct Selectors {
    code_segment: SegmentSelector,
    tss_segment: SegmentSelector,
}
