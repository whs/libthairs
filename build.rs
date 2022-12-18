////////////////////////////////////////////////////////////////////////////////
// Copyright (C) 2022 Manatsawin Hanmongkolchai
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; either
// version 2.1 of the License, or (at your option) any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
////////////////////////////////////////////////////////////////////////////////

use fst::SetBuilder;
use std::env;
use std::fs::{read_dir, File};
use std::io::{BufRead, BufReader, BufWriter};
use std::path::PathBuf;

fn build_data() {
    println!("cargo:rerun-if-changed=data");
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = PathBuf::from(out_dir);
    let out_file = out_path.join("thbrk.fst");
    let mut entries = Vec::new();

    for file in read_dir("data").unwrap() {
        let file = match file {
            Ok(f) => f,
            Err(_) => {
                continue;
            }
        };
        if !file
            .file_name()
            .to_str()
            .map(|t| Some(t.ends_with(".txt")))
            .is_some()
        {
            println!("skip file {}", file.path().to_string_lossy());
            continue;
        }
        if !file.metadata().map(|v| v.is_file()).is_ok() {
            println!("skip file {}", file.path().to_string_lossy());
            continue;
        }
        let fp = File::open(file.path());
        if fp.is_err() {
            println!(
                "cargo:warning=skippings file {}",
                file.path().to_string_lossy()
            );
            continue;
        }

        let fp = BufReader::new(fp.unwrap());
        entries.extend(fp.lines().filter(|v| v.is_ok()).map(|i| i.unwrap()));
    }

    entries.sort();

    let wtr = BufWriter::new(File::create(&out_file).unwrap());
    let mut builder = SetBuilder::new(wtr).unwrap();
    for entry in entries.into_iter() {
        builder.insert(entry.as_bytes()).unwrap();
    }
    builder.finish().unwrap();
}

fn main() {
    build_data();
}
