// Targets the Cortex-M33 processor (Armv8-M Mainline architecture profile),
// with the Floating Point extension.

use crate::spec::{
    Arch, Cc, CfgAbi, Env, FloatAbi, LinkerFlavor, Lld, Os, RelocModel, Target, TargetMetadata,
    TargetOptions, TlsModel, base, cvs,
};

pub(crate) fn target() -> Target {
    let mut base = base::arm_none::opts();
    base.add_pre_link_args(
        LinkerFlavor::Gnu(Cc::Yes, Lld::No),
        &["-mfloat-abi=hard", "-march=armv8-m.main+fp"],
    );
    Target {
        llvm_target: "thumbv8m.main-none-eabihf".into(),
        metadata: TargetMetadata {
            description: Some("vivo BlueOS on ARMv8-M Mainline, hardfloat".into()),
            tier: Some(3),
            host_tools: Some(false),
            std: Some(true),
        },
        pointer_width: 32,
        data_layout: "e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64".into(),
        arch: Arch::Arm,
        options: TargetOptions {
            families: cvs!["unix"],
            vendor: "vivo".into(),
            os: Os::Other("blueos".into()),
            env: Env::Newlib,
            cfg_abi: CfgAbi::EabiHf,
            features: "+fp-armv8d16sp".into(),
            llvm_floatabi: Some(FloatAbi::Hard),
            tls_model: TlsModel::Emulated,
            max_atomic_width: Some(32),
            linker: Some("arm-none-eabi-gcc".into()),
            linker_flavor: LinkerFlavor::Gnu(Cc::Yes, Lld::No),
            relocation_model: RelocModel::Pic,
            dynamic_linking: true,
            ..base
        },
    }
}
