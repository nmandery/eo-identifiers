use std::fs::read_to_string;

pub(crate) fn read_samples_from_txt(filename: &str) -> Vec<String> {
    let txt = format!("{}/testdata/{}", env!("CARGO_MANIFEST_DIR"), filename);
    read_to_string(txt)
        .unwrap()
        .split('\n')
        .flat_map(|line| {
            let line = line.trim().to_string();
            if line.starts_with('#') || line.is_empty() {
                None
            } else {
                Some(line)
            }
        })
        .collect()
}

pub(crate) fn apply_to_samples_from_txt<F>(filename: &str, f: F)
where
    F: Fn(&str),
{
    for sample in read_samples_from_txt(filename) {
        // dbg!(&sample);
        f(&sample);
    }
}
