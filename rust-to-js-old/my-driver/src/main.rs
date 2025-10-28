#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_middle;

use rustc_driver::{Callbacks, Compilation};
use rustc_interface::{Config, interface::Compiler};
use rustc_middle::ty::TyCtxt;

struct MyCallbacks;

impl Callbacks for MyCallbacks {
    fn config(&mut self, _config: &mut Config) {
        println!("My custom driver is running for a compilation!");
    }

    fn after_analysis<'tcx>(&mut self, _compiler: &Compiler, tcx: TyCtxt<'tcx>) -> Compilation {
        for id in tcx.hir_body_owners() {
            println!("{:?}", id);
        }

        Compilation::Continue
    }
}

fn main() {
    // 1. 커맨드 라인 인자를 수집합니다.
    let args: Vec<String> = std::env::args().collect();

    // Cargo는 RUSTC_WORKSPACE_WRAPPER를 다음과 같이 호출합니다:
    // /path/to/my-driver rustc --crate-name ...
    // 따라서 args[0]은 my-driver, args[1]은 'rustc' 또는 실제 rustc 경로가 됩니다.

    // 2. Cargo의 정보 조회 요청인지 확인합니다.
    // 인자 중에 '--print='로 시작하는 것이 있으면 정보 조회 요청입니다.
    let is_print_query = args.iter().any(|arg| arg.starts_with("--print="));

    if is_print_query {
        // 3. 정보 조회 요청이면, 우리는 아무것도 하지 않고 실제 rustc를 호출합니다.
        // args[1]은 실제 rustc의 경로이고, args[2..]가 rustc에 전달될 인자입니다.
        let rustc_path = &args[1];
        let rustc_args = &args[2..];

        let status = std::process::Command::new(rustc_path)
            .args(rustc_args)
            .status()
            .expect("failed to execute rustc");

        // rustc의 종료 코드로 그대로 종료합니다.
        std::process::exit(status.code().unwrap_or(1));
    } else {
        println!("{:?}", args);
        // 4. 실제 컴파일 요청이면, 우리의 Callbacks를 사용하여 컴파일러를 실행합니다.
        // rustc_driver는 'rustc' 경로를 포함한 전체 인자를 받기를 기대하므로
        // args[1] 부터 끝까지를 넘겨줍니다.
        rustc_driver::run_compiler(&args[1..], &mut MyCallbacks);
    }
}
