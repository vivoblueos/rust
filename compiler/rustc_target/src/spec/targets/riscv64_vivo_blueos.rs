use crate::spec::{
    Cc, CodeModel, LinkerFlavor, Lld, PanicStrategy, RelocModel, SanitizerSet, Target,
    TargetOptions, TlsModel, cvs,
};

pub(crate) fn target() -> Target {
    Target {
        data_layout: "e-m:e-p:64:64-i64:64-i128:128-n32:64-S128".into(),
        metadata: crate::spec::TargetMetadata {
            description: Some("vivo BlueOS on Bare RISC-V (RV64IMAC ISA)".into()),
            tier: Some(3),
            host_tools: Some(false),
            std: Some(true),
        },
        llvm_target: "riscv64".into(),
        pointer_width: 64,
        arch: "riscv64".into(),

        options: TargetOptions {
            families: cvs!["unix"],
            vendor: "vivo".into(),
            os: "blueos".into(),
            env: "newlib".into(),
            tls_model: TlsModel::Emulated,
            linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
            linker: Some("rust-lld".into()),
            llvm_abiname: "lp64".into(),
            cpu: "generic-rv64".into(),
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
