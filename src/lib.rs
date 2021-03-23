#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::{fs::File, io::{BufRead, BufReader}};

use napi::{CallContext, JsBoolean, JsObject, JsString, Result};

#[cfg(all(unix, not(target_env = "musl"), not(target_arch = "aarch64")))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[cfg(all(windows, target_arch = "x86_64"))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
    exports.create_named_method("parse", parse)?;

    Ok(())
}

#[js_function(2)]
fn parse(ctx: CallContext) -> Result<JsObject> {
    let path = ctx.get::<JsString>(0)?.into_utf8()?;
    let options = ctx.try_get::<JsObject>(1)?;

    let mut seperator = ',';
    let mut header = true;

    match options {
        // JsObject
        napi::Either::A(opt) => {
            if opt.has_named_property("seperator")? {
                let sep: JsString = opt.get_named_property("seperator")?;
                seperator = sep.into_utf8()?.as_str()?.chars().next().expect("Seperator should be a single char.");
            }

            if opt.has_named_property("header")? {
                let head: JsBoolean = opt.get_named_property("header")?;
                header = head.get_value()?;
            }
        },
        // JsUnknown
        napi::Either::B(_) => {},
    };

    let mut rows = ctx.env.create_array()?;

    let file = File::open(path.as_str()?)?;
    let buffered = BufReader::new(file);

    let mut headers: Vec<String> = vec![];

    for (row_index, row) in buffered.lines().enumerate() {
        let mut columns = ctx.env.create_array()?;
        for (column_index, column_value) in row?.as_str().split(seperator).enumerate() {
            if header {
                // first line
                if row_index == 0 {
                    headers.push(String::from(strip_bom(column_value)));
                } else {
                    columns.set_named_property(headers[column_index].as_str(), ctx.env.create_string(column_value)?)?;
                }
            } else {
                columns.set_element(column_index as u32, ctx.env.create_string(column_value)?)?;
            }
        }
        if header {
            // skip first line
            if row_index > 0 {
                // subtract first line
                rows.set_element(row_index as u32 - 1, columns)?;
            }
        } else {
            rows.set_element(row_index as u32, columns)?;
        }
    }

    Ok(rows)
}

fn strip_bom(string: &str) -> &str {
    string.trim_start_matches("\u{feff}")
}
