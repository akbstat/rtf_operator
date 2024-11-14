use std::{
    fs::{read, remove_file, OpenOptions},
    io::Write,
    ops::Sub,
    path::{Path, PathBuf},
};

use crate::utils::{
    misc::pattern_position,
    symbol::{PAGE_PAR, WINDOW_CTRL},
};

/// combine muliple rtfs into one rtf
pub fn combine(source: &[PathBuf], destination: &Path) -> anyhow::Result<()> {
    // create destination file
    if destination.exists() {
        remove_file(destination)?;
    }
    let mut destination = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(destination)?;

    // record if header is writen
    let mut header_writen = false;
    for (index, p) in source.iter().enumerate() {
        let data = read(p)?;
        if let Some((start, end)) = extract_file_content(&data) {
            if !header_writen {
                destination.write(data.get(0..start).unwrap())?;
                header_writen = true;
            }
            destination.write(data.get(start..end).unwrap())?;
            if index.lt(&source.len().sub(&1)) {
                destination.write(&PAGE_PAR)?;
            }
        }
    }
    destination.write(br"}")?;
    Ok(())
}

/// extract content of rtf, start from symbol "\widowctrl", end with the next to last charater
/// ```rust
/// #[test]
/// fn extract_test() {
///     let data = br"\widowctrl\test}}";
///     let result = extract_file_content(&data);
///     assert_eq(result, Some((0, data.len() - 3)));
/// }
/// ```
fn extract_file_content(data: &[u8]) -> Option<(usize, usize)> {
    let mut last_curly_brace = data.len() - 1;
    // seek the last curly brace
    while last_curly_brace.gt(&0) {
        if let Some(char) = data.get(last_curly_brace) {
            if char.eq(&b'}') {
                break;
            }
            last_curly_brace -= 1;
        }
    }
    match pattern_position(&WINDOW_CTRL, &data, 0) {
        Some((start, _)) => Some((start, last_curly_brace)),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn combine_test() -> anyhow::Result<()> {
        let source = vec![
            Path::new("D:\\Studies\\ak101\\203\\stats\\dryrun\\product\\output\\t-14-01-01-01-disp-scr.rtf").into(),
            Path::new("D:\\Studies\\ak101\\203\\stats\\dryrun\\product\\output\\t-14-01-01-02-sf-scr.rtf").into(),
            Path::new("D:\\Studies\\ak101\\203\\stats\\dryrun\\product\\output\\t-14-01-02-pd-fas.rtf").into(),
            Path::new("D:\\Studies\\ak101\\203\\stats\\dryrun\\product\\output\\t-14-01-03-01-dm-fas.rtf").into(),
            Path::new("D:\\Studies\\ak101\\203\\stats\\dryrun\\product\\output\\t-14-01-03-02-baseline-fas.rtf").into(),
        ];
        let destination =
            Path::new("D:\\Studies\\ak101\\203\\stats\\dryrun\\product\\output\\combined\\rtf.rtf");
        combine(&source, &destination)?;
        Ok(())
    }
}
