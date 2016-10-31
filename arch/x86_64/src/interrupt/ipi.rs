use device::local_apic::LOCAL_APIC;
use x86::tlb;

interrupt!(flush, {
    tlb::flush_all();
    LOCAL_APIC.eoi();
});

interrupt!(wakeup, {
    LOCAL_APIC.eoi();
});
