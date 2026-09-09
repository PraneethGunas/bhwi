#[cfg(feature = "bitbox")]
use bhwi::bitbox::{ManagementContext, SetupMode};
#[cfg(any(feature = "bitbox", feature = "keepkey", feature = "trezor"))]
use bhwi::common::DeviceContext;

#[cfg(feature = "trezor")]
pub fn trezor_setup_context(host_entropy: [u8; 32]) -> DeviceContext {
    DeviceContext::TrezorManagement(bhwi::trezor::ManagementContext::Setup { host_entropy })
}

#[cfg(feature = "trezor")]
pub fn trezor_pin_context(pin: bhwi::trezor::HostPin) -> DeviceContext {
    DeviceContext::TrezorManagement(bhwi::trezor::ManagementContext::Pin(pin))
}

#[cfg(feature = "trezor")]
pub fn trezor_restore_context(u2f_counter: u32) -> DeviceContext {
    DeviceContext::TrezorManagement(bhwi::trezor::ManagementContext::Restore { u2f_counter })
}

#[cfg(feature = "bitbox")]
pub fn bitbox_setup_context(
    mode: SetupMode,
    timestamp: u32,
    timezone_offset: i32,
) -> DeviceContext {
    DeviceContext::BitBoxManagement(ManagementContext::Setup {
        mode,
        timestamp,
        timezone_offset,
    })
}

#[cfg(feature = "bitbox")]
pub fn bitbox_restore_context(timestamp: u32, timezone_offset: i32) -> DeviceContext {
    DeviceContext::BitBoxManagement(ManagementContext::Restore {
        timestamp,
        timezone_offset,
    })
}

#[cfg(feature = "keepkey")]
pub fn keepkey_setup_context(host_entropy: [u8; 32]) -> DeviceContext {
    DeviceContext::KeepKeyManagement(bhwi::keepkey::ManagementContext::Setup { host_entropy })
}

#[cfg(feature = "keepkey")]
pub fn keepkey_pin_context(pin: bhwi::keepkey::HostPin) -> DeviceContext {
    DeviceContext::KeepKeyManagement(bhwi::keepkey::ManagementContext::Pin(pin))
}

#[cfg(feature = "keepkey")]
pub fn keepkey_restore_context(u2f_counter: u32) -> DeviceContext {
    DeviceContext::KeepKeyManagement(bhwi::keepkey::ManagementContext::Restore { u2f_counter })
}
