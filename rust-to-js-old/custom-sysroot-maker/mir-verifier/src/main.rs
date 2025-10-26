#![feature(rustc_private)]
#![feature(box_patterns)]

extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_hir;

use rustc_driver::Compilation;
use rustc_interface::interface;
use rustc_middle::ty::TyCtxt;
use std::env;

struct MirVerifier;

impl rustc_driver::Callbacks for MirVerifier {
    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &interface::Compiler,
        tcx: TyCtxt<'tcx>,
    ) -> Compilation {
        println!("\n=== MIR Verification Started ===\n");

        // Step 1: Print MIR for local items (the test code)
        println!("Step 1: Extracting MIR from test code (let a = 5; println!(\"{{}}\", a);)\n");

        for local_def_id in tcx.mir_keys(()) {
            let def_id = local_def_id.to_def_id();
            let def_path = tcx.def_path_str(def_id);

            println!("Local Item: {}", def_path);

            // Get the optimized MIR (this is safer than mir_built which can be stolen)
            let mir = tcx.optimized_mir(def_id);
            println!("Optimized MIR for {}:", def_path);
            println!("{:#?}\n", mir);
        }

        // Step 2: Try to access MIR from std library
        println!("\n=== Step 2: Available crates ===\n");

        // List all available crates
        for crate_num in tcx.crates(()) {
            let crate_name = tcx.crate_name(*crate_num);
            println!("  - {}", crate_name);
        }

        // Step 3: Try to find i32's Display::fmt implementation
        println!("\n=== Step 3: Searching for i32's Display::fmt implementation ===\n");

        // Get all local trait implementations (these are from our test code only)
        let all_trait_impls = tcx.all_local_trait_impls(());
        println!("Total local trait implementations: {}", all_trait_impls.len());

        // Step 4: Try to access external items
        println!("\n=== Step 4: Attempting to access external crate items ===\n");

        // We need to search through external crates
        // Try to iterate through all DefIds from core/std

        use rustc_hir::def_id::{DefId, LOCAL_CRATE};
        use rustc_middle::ty::TyKind;

        // Get the core crate
        for crate_num in tcx.crates(()) {
            let crate_name = tcx.crate_name(*crate_num);

            if crate_name.as_str() == "core" {
                println!("Found core crate: {:?}", crate_num);

                // Try to get all items from the core crate
                // This is tricky - we need to use specific APIs

                // One approach: look for specific known paths
                // i32's Display impl should be in core::fmt::num

                println!("Searching for items in core crate...");

                // Try to resolve a path manually
                // We know println! calls core::fmt::rt::Argument::new_display::<i32>
            }
        }

        // Step 5: More direct approach - analyze the MIR we already have
        println!("\n=== Step 5: Analyzing function calls in our MIR ===\n");

        println!("From our test code's MIR, we can see it calls:");
        println!("  - core::fmt::rt::Argument::<'_>::new_display::<i32>");
        println!("  - core::fmt::rt::<impl Arguments<'_>>::new_v1");
        println!("\nLet's try to find these functions and check their MIR...\n");

        // Try a more comprehensive search
        // Iterate through all possible DefIds from external crates

        use rustc_span::Symbol;

        for crate_num in tcx.crates(()) {
            let crate_name = tcx.crate_name(*crate_num);

            // Focus on core where primitive implementations are
            if crate_name.as_str() != "core" {
                continue;
            }

            println!("Examining core crate for Display/fmt implementations...");

            // Try to get external items using different APIs
            // Note: This might not work for all items due to metadata limitations

            // Check if we can iterate crate items
            // (This API may not be available or may require different approach)

            println!("Note: Direct iteration of external crate items requires");
            println!("      special compiler APIs that may not be easily accessible.");
            break;
        }

        // Step 6: Try to directly access specific std functions using instance resolution
        println!("\n=== Step 6: Resolving and accessing std library function MIR ===\n");

        use rustc_middle::ty::{Instance, ParamEnv};

        // Try to resolve the function we know is called: Argument::new_display
        // We can try to resolve it from a known path

        // First, let's try to iterate all available instances
        println!("Attempting to access MIR for known std functions...\n");

        // Get DefIds from our local MIR's callees
        for local_def_id in tcx.mir_keys(()) {
            let def_id = local_def_id.to_def_id();
            let mir = tcx.optimized_mir(def_id);

            // Iterate through all basic blocks and find function calls
            for (bb_idx, bb_data) in mir.basic_blocks.iter_enumerated() {
                use rustc_middle::mir::TerminatorKind;

                if let Some(ref terminator) = bb_data.terminator {
                    if let TerminatorKind::Call { func, .. } = &terminator.kind {
                        use rustc_middle::mir::Operand;
                        use rustc_middle::ty::TyKind;

                        // Get the function being called
                        if let Operand::Constant(box constant) = func {
                            let const_ty = constant.const_.ty();

                            if let TyKind::FnDef(callee_def_id, substs) = const_ty.kind() {
                                let callee_path = tcx.def_path_str(*callee_def_id);

                                // Check if this is a std/core function
                                if callee_path.starts_with("core::") || callee_path.starts_with("std::") {
                                    println!("Found call to std function: {}", callee_path);
                                    println!("  DefId: {:?}", callee_def_id);

                                    // THIS IS THE KEY TEST: Try to access MIR for this std function
                                    if tcx.is_mir_available(*callee_def_id) {
                                        println!("  ✓✓✓ MIR IS AVAILABLE! ✓✓✓");

                                        let std_mir = tcx.optimized_mir(*callee_def_id);
                                        println!("  MIR has {} basic blocks", std_mir.basic_blocks.len());
                                        println!("  First 100 chars of MIR debug output:");

                                        let mir_debug = format!("{:#?}", std_mir);
                                        let preview = &mir_debug[..mir_debug.len().min(500)];
                                        println!("{}", preview);
                                        println!("  ...\n");

                                        // Special check for i32's Display::fmt
                                        if callee_path.contains("i32") && callee_path.contains("Display") {
                                            println!("  *** THIS IS i32's Display::fmt! ***");
                                            println!("  Full MIR:");
                                            println!("{:#?}", std_mir);
                                        }
                                    } else {
                                        println!("  ✗ MIR is NOT available");
                                        println!("    This means -Zalways-encode-mir did NOT work for this function");
                                    }
                                    println!();
                                }
                            }
                        }
                    }
                }
            }
        }

        println!("\nSummary:");
        println!("-------");
        println!("✓ Successfully extracted MIR from our test code using tcx.optimized_mir()");
        println!("✓ Found function calls to std/core functions");
        println!("? Checked if MIR is available for std/core functions");
        println!("\nThe key question: Can we access tcx.optimized_mir() for i32's Display::fmt?");
        println!("This is the definitive test for -Zalways-encode-mir effectiveness.");

        println!("\n=== MIR Verification Completed ===\n");

        Compilation::Stop
    }
}

fn main() {
    // Create a temporary test file
    let test_code = r#"
fn main() {
    let a = 5;
    println!("{}", a);
}
"#;

    let temp_dir = std::env::temp_dir();
    let test_file = temp_dir.join("test_mir.rs");
    std::fs::write(&test_file, test_code).expect("Failed to write test file");

    // Get the custom sysroot path
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let sysroot_path = current_dir.parent().unwrap().join("custom-sysroot");

    println!("Using custom sysroot: {}", sysroot_path.display());
    println!("This sysroot was built with -Zalways-encode-mir flag\n");

    // Prepare rustc arguments
    let args: Vec<String> = vec![
        "rustc".to_string(),
        test_file.to_str().unwrap().to_string(),
        "--crate-type=bin".to_string(),
        "--target=wasm32-wasip1-threads".to_string(),
        format!("--sysroot={}", sysroot_path.display()),
        "-Zalways-encode-mir".to_string(),
    ];

    // Run the compiler with our callback
    let mut callbacks = MirVerifier;

    let result = rustc_driver::catch_fatal_errors(|| {
        rustc_driver::run_compiler(&args, &mut callbacks)
    });

    std::process::exit(match result {
        Ok(_) => 0,
        Err(_) => 1,
    });
}
