#[inline(always)]
pub fn yield_now() {
    unsafe { nds_sys::cothread_yield() };
}
