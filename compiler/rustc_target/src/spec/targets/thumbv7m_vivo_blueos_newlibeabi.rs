// Targets the Cortex-M4 processor (ARMv7-M)

use crate::spec::{Cc, LinkerFlavor, Lld, Target, TargetOptions, TlsModel, base, cvs};

pub(crate) fn target() -> Target {
    Target {
        llvm_target: "thumbv7m-none-eabi".into(),
        metadata: crate::spec::TargetMetadata {
            description: Some("vivo BlueOS on Thumbv7m".into()),
            tier: Some(3),
            host_tools: Some(false),
            std: Some(true),
        },
        pointer_width: 32,
        data_layout: "e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64".into(),
        arch: "arm".into(),
        options: TargetOptions {
            families: cvs!["unix"],
            vendor: "vivo".into(),
            os: "blueos".into(),
            abi: "eabi".into(),
            env: "newlib".into(),
            tls_model: TlsModel::Emulated,
            max_atomic_width: Some(32),
            linker: Some("arm-none-eabi-gcc".into()),
            linker_flavor: LinkerFlavor::Gnu(Cc::Yes, Lld::No),
            ..base::thumb::opts()
        },
    }
}
