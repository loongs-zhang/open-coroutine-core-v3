// Copyright 2016 coroutine-rs Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

fn main() {
    #[cfg(any(feature = "setjmp", feature = "boost"))]
    {
        extern crate cc;
        use std::env;
        use std::path::PathBuf;

        let target: String = env::var("TARGET").unwrap();
        let is_win_gnu = target.ends_with("windows-gnu");
        let is_win_msvc = target.ends_with("windows-msvc");
        let is_win = is_win_gnu || is_win_msvc;

        let arch = match target.split('-').next().unwrap() {
            "arm" | "armv7" | "armv7s" => "arm",
            "arm64" | "aarch64" => "arm64",
            "x86" | "i386" | "i486" | "i586" | "i686" => "i386",
            "mips" | "mipsel" => "mips32",
            "powerpc" => "ppc32",
            "powerpc64" => "ppc64",
            "x86_64" => "x86_64",
            _ => panic!("Unsupported architecture: {target}"),
        };

        let abi = match arch {
            "arm" | "arm64" => "aapcs",
            "mips32" => "o32",
            _ => {
                if is_win {
                    "ms"
                } else {
                    "sysv"
                }
            }
        };

        let format = if is_win {
            "pe"
        } else if target.contains("apple") {
            "macho"
        } else if target.ends_with("aix") {
            "xcoff"
        } else {
            "elf"
        };

        let (asm, ext) = if is_win_msvc {
            if arch == "arm" {
                ("armasm", "asm")
            } else {
                ("masm", "asm")
            }
        } else if is_win_gnu {
            ("gas", "asm")
        } else {
            ("gas", "S")
        };

        cfg_if::cfg_if! {
            if #[cfg(feature = "setjmp")] {
                let prefixes = ["long", "set"];
                let base_path: PathBuf = ["src", "context", "setjmp", "asm"].iter().collect();
            } else if #[cfg(feature = "boost")] {
                let prefixes = ["jump", "make"];
                let base_path: PathBuf = ["src", "context", "boost", "asm"].iter().collect();
            } else {
                compile_error!("Unsupported feature");
            }
        }

        let mut config = cc::Build::new();

        config.define("BOOST_CONTEXT_EXPORT", None);
        if is_win_gnu {
            config.flag("-x").flag("assembler-with-cpp");
        }

        for prefix in prefixes.iter() {
            let file_name: [&str; 11] =
                [prefix, "_", arch, "_", abi, "_", format, "_", asm, ".", ext];
            let file_name = file_name.concat();

            let mut path = base_path.clone();
            path.push(file_name);
            config.file(path.to_str().unwrap());
        }

        cfg_if::cfg_if! {
            if #[cfg(feature = "setjmp")] {
                config.compile("libsetjmp.a");
            } else if #[cfg(feature = "boost")] {
                config.compile("libboost_context.a");
            } else {
                compile_error!("Unsupported feature");
            }
        }
    }
}
