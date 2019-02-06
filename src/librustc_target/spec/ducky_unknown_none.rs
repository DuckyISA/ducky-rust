// Generic Ducky target for bare-metal code

use super::{LldFlavor, LinkerFlavor, Target, TargetOptions, PanicStrategy};

pub fn target() -> Result<Target, String> {
    let opts = TargetOptions {
        linker: Some("rust-lld".to_owned()),
        relocation_model: "static".to_string(),
        disable_redzone: true,
        linker_is_gnu: true,
        max_atomic_width: Some(32),
        panic_strategy: PanicStrategy::Abort,
        dynamic_linking: false,
        executables: true,
        has_elf_tls: false,
        no_integrated_as: false,
        atomic_cas: true,
        i128_lowering: true,
        .. Default::default()
    };
    Ok(Target {
        llvm_target: "ducky-unknown-none".to_string(),
        target_endian: "little".to_string(),
        target_pointer_width: "32".to_string(),
        target_c_int_width: "32".to_string(),
        target_os: "none".to_string(),
        target_env: String::new(),
        target_vendor: String::new(),
        data_layout: "e-S32-m:e-p:32:32-i1:8:8-i8:8:8-i16:16:16-i32:32:32-i64:64:64-a:0:32-n32".to_string(),
        arch: "ducky".to_string(),
        linker_flavor: LinkerFlavor::Lld(LldFlavor::Ld),
        options: opts,
    })
}
