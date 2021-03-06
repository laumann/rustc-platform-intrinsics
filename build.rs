extern crate platform_intrinsics;

use platform_intrinsics::IntrinsicsInput;
use std::fs::File;

fn main() {

    let arm = IntrinsicsInput::single("arch/arm.json");

    let x86 = IntrinsicsInput::multi("arch/x86/info.json", vec![
        "arch/x86/avx.json",
        "arch/x86/avx2.json",
        "arch/x86/bmi.json",
        "arch/x86/bmi2.json",
        "arch/x86/fma.json",
        "arch/x86/rdrand.json",
        "arch/x86/rdseed.json",
        "arch/x86/sse.json",
        "arch/x86/sse2.json",
        "arch/x86/sse3.json",
        "arch/x86/sse41.json",
        "arch/x86/sse42.json",
        "arch/x86/ssse3.json",
        "arch/x86/tbm.json",
    ]);

    let intrinsics = vec![
        ("src/x86.rs", x86),
        ("src/arm.rs", arm),
        ("src/aarch64.rs", IntrinsicsInput::single("arch/aarch64.json")),
        ("src/nvptx.rs", IntrinsicsInput::multi("arch/nvptx/info.json", vec![
                                                    "arch/nvptx/cuda.json",
                                                    "arch/nvptx/sreg.json",
                                                ])),
        ("src/hexagon.rs", IntrinsicsInput::single("arch/hexagon/hvx_v60.json")),
        ("src/powerpc.rs", IntrinsicsInput::single("arch/powerpc.json")),
    ];

    for (output, input) in intrinsics {
        let mut o = File::create(output)
            .expect(&format!("Unable to create file '{}'", output));

        platform_intrinsics::generate(input, &mut o)
            .expect(&format!("Unable to generate '{}'", output));
    }

}
