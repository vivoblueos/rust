use crate::spec::{
    Cc, CodeModel, LinkerFlavor, Lld, PanicStrategy, RelocModel, SanitizerSet, Target,
    TargetOptions, TlsModel, cvs,
};

pub(crate) fn target() -> Target {
    Target {
        data_layout: "e-m:e-p:32:32-i64:64-n32-S128".into(),
        metadata: crate::spec::TargetMetadata {
            description: Some("vivo BlueOS on Bare RISC-V (RV32IMAC ISA)".into()),
            tier: Some(3),
            host_tools: Some(false),
            std: Some(true),
        },
        llvm_target: "riscv32".into(),
        pointer_width: 32,
        arch: "riscv32".into(),

        options: TargetOptions {
            families: cvs!["unix"],
            vendor: "vivo".into(),
            os: "blueos".into(),
            env: "newlib".into(),
            tls_model: TlsModel::Emulated,
            linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
            linker: Some("rust-lld".into()),
            llvm_abiname: "ilp32".into(),
            cpu: "generic-rv32".into(),
            max_atomic_width: Some(64),
            features: "+m,+a,+c".into(),
            panic_strategy: PanicStrategy::Abort,
            relocation_model: RelocModel::Static,
            code_model: Some(CodeModel::Medium),
            emit_debug_gdb_scripts: false,
            eh_frame_header: false,
            supported_sanitizers: SanitizerSet::KERNELADDRESS | SanitizerSet::SHADOWCALLSTACK,
            ..Default::default()
        },
    }
}
