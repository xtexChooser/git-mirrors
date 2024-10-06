#pragma once

extern "C" {
namespace yztsec {

bool yztsec_check_peb_debugged();
bool yztsec_check_intr3();
bool yztsec_check_dr_regs();
bool yztsec_check_trap();

} // namespace yztsec
}
