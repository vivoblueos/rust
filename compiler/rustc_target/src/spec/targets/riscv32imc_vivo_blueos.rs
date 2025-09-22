use crate::spec::{PanicStrategy, RelocModel, Target, TargetOptions, TlsModel, cvs};

pub(crate) fn target() -> Target {
    Target {
        data_layout: "e-m:e-p:32:32-i64:64-n32-S128".into(),
        llvm_target: "riscv32".into(),
        metadata: crate::spec::TargetMetadata {
            description: Some("vivo BlueOS on ESP32 RISC-V SoC".into()),
            tier: Some(3),
            host_tools: Some(false),
            std: Some(true),
        },
        pointer_width: 32,
        arch: "riscv32".into(),
        options: TargetOptions {
            families: cvs!["unix"],
            os: "blueos".into(),
            env: "newlib".into(),
            tls_model: TlsModel::Emulated,
            vendor: "vivo".into(),
            linker: Some("rust-lld".into()),
            cpu: "generic-rv32".into(),
            // While the RiscV32IMC architecture does not natively
            // support atomics, BlueKernel does support the __atomic*
            // GCC builtins, so setting `max_atomic_width` to
            // `Some(32)` and `atomic_cas` to `true` will cause the
            // compiler to emit libcalls to these builtins.
            //
            // Support for atomics is necessary for the Rust STD
            // library, which is supported by the BlueKernel.
            max_atomic_width: Some(32),
            atomic_cas: true,
            features: "+m,+c".into(),
            llvm_abiname: "ilp32".into(),
            panic_strategy: PanicStrategy::Abort,
            relocation_model: RelocModel::Static,
            emit_debug_gdb_scripts: false,
            eh_frame_header: false,
            ..Default::default()
        },
    }
}
