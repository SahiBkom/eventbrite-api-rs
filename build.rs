use ::api_blueprint_to_rust::ApibToRs;
use patch::{Line, Patch};
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("eventbriteapiv3public.rs");
    let mut f = File::create(&dest_path).unwrap();
    // wget https://jsapi.apiary.io/apis/eventbriteapiv3public.source
    let s = ApibToRs::new(String::from("./api/eventbriteapiv3public.source"))
        .rs_as_string()
        .unwrap();

    // let patch = std::fs::read_to_string("./patch/fix_pagination_continuation.patch").unwrap();
    // let diff = Patch::from_single(&patch).unwrap();
    // let new = apply(diff, &s);

    f.write_all(s.as_bytes()).unwrap();

    // diff -u ./patch/v1.rs patch/v2.rs > fix_pagination.patch
    if let Some(dest) = dest_path.to_str() {
        Command::new("patch")
            .args(&[dest, "./patch/fix_pagination.patch"])
            .output()
            .expect("failed to execute process");
    }
}

// fn apply(diff: Patch, old: &str) -> String {
//     let old_lines = old.lines().collect::<Vec<&str>>();
//     let mut out: Vec<&str> = vec![];
//     let mut old_line = 0;
//     for hunk in diff.hunks {
//         while old_line < hunk.old_range.start - 1 {
//             out.push(old_lines[old_line as usize]);
//             old_line += 1;
//         }
//         old_line += hunk.old_range.count;
//         for line in hunk.lines {
//             match line {
//                 Line::Add(s) | Line::Context(s) => out.push(s),
//                 Line::Remove(_) => {}
//             }
//         }
//     }
//     out.join("\n")
// }
