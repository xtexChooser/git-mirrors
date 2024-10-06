extern "C" {
    #[link_name = "yztsec_check_peb_debugged"]
    pub fn check_peb_debugged() -> bool;

    #[link_name = "yztsec_check_intr3"]
    pub fn check_intr3() -> bool;

    #[link_name = "yztsec_check_dr_regs"]
    pub fn check_dr_regs() -> bool;

    #[link_name = "yztsec_check_trap"]
    pub fn check_trap() -> bool;
}
