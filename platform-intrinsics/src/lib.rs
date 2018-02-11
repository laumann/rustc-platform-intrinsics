extern crate serde_json;
extern crate regex;
extern crate lazy_static;

mod typespec;

use std::io::{self, Write};
use std::path::Path;
use std::convert::AsRef;
use std::fs::File;
use std::collections::HashMap;

/// There are two different operating modules: single file and multiple files.
pub enum IntrinsicsInput<P: AsRef<Path>> {

    /// A single file that contains both platform information and intrinsics
    /// specification.
    Single(P),

    /// The first file is the platoform information, the remaining list of files
    /// are the intrinsics specification
    Multi(P, Vec<P>),
}

impl<P: AsRef<Path>> IntrinsicsInput<P> {

    pub fn single(p: P) -> IntrinsicsInput<P> {
        IntrinsicsInput::Single(p)
    }

    pub fn multi(p: P, ps: Vec<P>) -> IntrinsicsInput<P> {
        IntrinsicsInput::Multi(p, ps)
    }
}

/// Generate 
/// FIXME(laumann): Add support for multiple output formats (extern-block, compiler-defs)
pub fn generate<P, W>(input: IntrinsicsInput<P>, f: &mut W) -> io::Result<()>
    where P: AsRef<Path>,
          W: Write
{
    // Ok, here are the steps
    // (1) Parse info file and gather platform information
    // (2) Iterate all intrinsic specs and output intrinsics
    f.write("// Auto-generated module!\n".as_bytes())?;
    match input {
        IntrinsicsInput::Single(p) => {
            let pf = extract_platform_info(p.as_ref())?;
            write!(f, "// {:?}\n", pf)?;
            write!(f, "// Single file: {}\n", p.as_ref().display())?;
        }
        IntrinsicsInput::Multi(p, ps) => {
            let pf = extract_platform_info(p.as_ref())?;
            write!(f, "// Platform: {:?}\n", pf)?;
            write!(f, "// Info: {}\n", p.as_ref().display())?;
            for q in ps {
                write!(f, "// File: {}\n", q.as_ref().display())?;
            }
        }
    }
    
    Ok(())
}

#[derive(Debug)]
struct PlatformInfo {
    platform: String,
    widths: HashMap<u32, HashMap<String, String>>,
    number_info: HashMap<String, HashMap<String, String>>,
}

fn extract_platform_info(p: &Path) -> io::Result<PlatformInfo> {
    // Read input file as JSON and extract the platform information
    //
    let f = File::open(p)?;
    let r: serde_json::Value = serde_json::from_reader(f)?;

    let platform = r["platform"].as_str()
        .expect("'platform' field not a string")
        .to_owned();

    let mut widths = HashMap::new();

    let width_info = &r["width_info"];
    if width_info.is_object() {
        for (key, val) in width_info.as_object().unwrap().iter() {
            let w = key.parse::<u32>().expect("'width_info' key field should be numeric");
            assert!(w.is_power_of_two());

            // Each val should be an object as well
            let mut width_info = HashMap::new();
            if val.is_object() {
                for (k, v) in val.as_object().unwrap().iter() {
                    width_info.insert(k.to_owned(), v.as_str().unwrap().to_owned());
                }
            }
            
            widths.insert(w, width_info);
        }
    }

    let mut type_info = HashMap::with_capacity(3);

    let number_info = &r["number_info"];
    if number_info.is_object() {
        for (key, val) in number_info.as_object().unwrap().iter() {
            let mut ninfo = HashMap::new();

            if val.is_object() {
                for (k, v) in val.as_object().unwrap().iter() {
                    // FIXME(laumann): v is either a Map or a String
                    println!("cargo:warning={}", v);
                    ninfo.insert(k.to_owned(), v.as_str().unwrap_or("").to_owned());
                }
            }

            type_info.insert(key.to_owned(), ninfo);
        }
    }

    Ok(PlatformInfo {
        platform: platform,
        widths: widths,
        number_info: type_info
    })
}
