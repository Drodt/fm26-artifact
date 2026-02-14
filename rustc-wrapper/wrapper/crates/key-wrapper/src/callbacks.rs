use std::fs;

use rml::{
    callbacks::{retrieve_rcx, store_rcx},
    ctx::RmlCtxt,
    spec::{SerializableSpecMap, SpecMap},
    suppress_borrowck, Options as RmlOpts,
};
use rustc_driver::{Callbacks, Compilation};
use rustc_interface::{interface::Compiler, Config};
use rustc_middle::ty::TyCtxt;
use serde::Serialize;

use crate::{
    convert,
    hir::Crate,
    options::{Options, OutputFile},
};

pub struct Wrapper {
    converted: Option<Crate>,
    specs: Option<SpecMap>,
    options: Options,
}

#[derive(Serialize)]
struct Output<'a> {
    #[serde(rename = "crate")]
    pub krate: &'a Crate,
    pub specs: SerializableSpecMap<'a>,
}

impl Wrapper {
    pub fn new(options: Options) -> Self {
        Wrapper {
            converted: None,
            specs: None,
            options,
        }
    }

    pub fn result(self) -> Crate {
        self.converted.unwrap()
    }

    pub fn print(&self) {
        let Some(out) = &self.options.output_file else {
            return;
        };
        let krate = self
            .converted
            .as_ref()
            .expect("`print` called before crate was converted");
        let output = Output {
            krate,
            specs: self
                .specs
                .as_ref()
                .expect("No specs extracted")
                .serializable(),
        };
        let json = if self.options.pretty_print {
            serde_json::to_string_pretty(&output)
        } else {
            serde_json::to_string(&output)
        }
        .expect("serialization error");

        match out {
            OutputFile::File(file) => fs::write(file, json).expect("writing failed"),
            OutputFile::Stdout => println!("{json}"),
        }
    }
}

impl Callbacks for Wrapper {
    fn config(&mut self, config: &mut Config) {
        config.override_queries = Some(|_, providers| {
            providers.mir_built = |tcx, did| {
                let mir = (rustc_interface::DEFAULT_QUERY_PROVIDERS.mir_built)(tcx, did);
                let mut mir = mir.steal();
                suppress_borrowck::suppress_borrowck(tcx, did.to_def_id(), &mut mir);
                tcx.alloc_steal_mir(mir)
            }
        });
    }

    fn after_expansion<'tcx>(
        &mut self,
        compiler: &rustc_interface::interface::Compiler,
        tcx: TyCtxt<'tcx>,
    ) -> Compilation {
        compiler.sess.dcx().abort_if_errors();

        let mut rcx = RmlCtxt::new(
            tcx,
            RmlOpts {
                should_output: false,
                output_file: None,
                in_cargo: true,
                print_expanded: false,
                print_specs_debug: false,
                pretty_print: false,
            },
        );
        rcx.validate();
        unsafe { store_rcx(rcx) };

        compiler.sess.dcx().abort_if_errors();

        Compilation::Continue
    }

    fn after_analysis<'tcx>(&mut self, _: &Compiler, tcx: TyCtxt<'tcx>) -> Compilation {
        eprintln!("Starting conversion");
        let rcx = unsafe { retrieve_rcx(tcx) };
        let specs = rcx.get_specs();
        //eprintln!("specs={specs:?}");
        self.specs = Some(specs);
        self.converted = Some(convert(tcx));
        self.print();

        Compilation::Continue
    }
}
