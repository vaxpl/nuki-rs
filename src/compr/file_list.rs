use crate::{vec2, Context, FlagsBuilder, Key, LayoutFormat, String as NkString};
use chrono::{DateTime, Local};
use std::cmp::Ordering;
use std::ffi::{OsStr, OsString};
use std::fs::{read_dir, DirEntry};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// A partial file information.
#[derive(Debug, PartialEq, Eq)]
pub struct FileInfo {
    /// The bare file name of this entry without any other leading path component.
    pub file_name: OsString,
    /// The full path to the file that this entry represents.
    pub path: PathBuf,
    /// The size of the file, in bytes.
    pub len: u64,
    /// The last modification time of this file.
    pub modified: SystemTime,
}

impl PartialOrd for FileInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.modified.partial_cmp(&other.modified)
    }
}

impl Ord for FileInfo {
    fn cmp(&self, other: &Self) -> Ordering {
        self.modified.cmp(&other.modified)
    }
}

/// A list of disk files.
#[derive(Debug)]
pub struct FileList {
    path: PathBuf,
    ext_filter: OsString,
    files: Vec<FileInfo>,
    selected: usize,
}

impl FileList {
    fn scan_files<P: AsRef<Path>, T: AsRef<OsStr>>(path: P, ext_filter: T) -> Vec<FileInfo> {
        let mut files: Vec<FileInfo> = vec![];
        let ext_filter = ext_filter.as_ref();
        let pattern_filter = |x: &Result<DirEntry, std::io::Error>| -> bool {
            if ext_filter.is_empty() || ext_filter == "*" {
                true
            } else {
                x.as_ref()
                    .map(|v| v.path().extension() == Some(ext_filter))
                    .unwrap_or(false)
            }
        };

        if let Ok(entries) = read_dir(path) {
            for entry in entries.filter(pattern_filter) {
                if let Ok(entry) = entry {
                    let (len, modified) = if let Ok(m) = entry.metadata() {
                        (m.len(), m.modified().unwrap_or(SystemTime::UNIX_EPOCH))
                    } else {
                        (0, SystemTime::UNIX_EPOCH)
                    };
                    files.push(FileInfo {
                        file_name: entry.file_name(),
                        path: entry.path(),
                        len,
                        modified,
                    });
                }
            }
        }

        // Sorting with modified order by desc.
        files.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());

        files
    }

    /// Construct a new file list.
    pub fn new<P: AsRef<Path>, T: AsRef<OsStr>>(path: P, ext_filter: T) -> Self {
        let files = Self::scan_files(&path, &ext_filter);
        Self {
            path: path.as_ref().to_path_buf(),
            ext_filter: ext_filter.as_ref().to_os_string(),
            files,
            selected: 0,
        }
    }

    /// Returns true if the list no files.
    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }

    /// Returns the number of files in the list.
    pub fn len(&self) -> usize {
        self.files.len()
    }

    /// Returns the file reference at index in the list.
    pub fn get(&self, index: usize) -> Option<&FileInfo> {
        self.files.get(index)
    }

    /// Returns an iterator over the slice.
    pub fn iter(&self) -> std::slice::Iter<'_, FileInfo> {
        self.files.iter()
    }

    /// Mark `prev` file as `selected`.
    pub fn select_prev(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    /// Mark `prev` file as `selected`, wrap to `last` file when current at `first` file.
    pub fn select_prev_wrapped(&mut self) {
        if self.selected == 0 {
            self.selected = self.len() - 1;
        } else {
            self.selected -= 1;
        }
    }

    /// Mark next file as `selected`.
    pub fn select_next(&mut self) {
        self.selected += 1;
        if self.selected >= self.len() {
            self.selected = self.len() - 1;
        }
    }

    /// Mark next file as `selected`, wrap to `first` file when current at `last` file.
    pub fn select_next_wrapped(&mut self) {
        self.selected += 1;
        if self.selected >= self.len() {
            self.selected = 0;
        }
    }

    /// Returns the `selected` file index.
    pub fn selected(&self) -> usize {
        self.selected
    }

    /// Returns the `selected` file information.
    pub fn selected_file(&self) -> Option<&FileInfo> {
        if self.is_empty() {
            None
        } else {
            self.get(self.selected)
        }
    }

    /// Clear the files and rescan with constructed `path` and `ext_filter`.
    pub fn refresh(&mut self) {
        self.files = Self::scan_files(&self.path, &self.ext_filter);
        self.selected = 0;
    }
}

/// A file list input controller.
#[derive(Debug)]
pub struct FileListInputCtrl;

impl Default for FileListInputCtrl {
    fn default() -> Self {
        Self::new()
    }
}

impl FileListInputCtrl {
    /// Construct a new input controller for file list.
    pub fn new() -> Self {
        Self {}
    }

    /// Processing input events.
    pub fn process(self, ctx: &Context, fb: &mut FileList) {
        let input = ctx.input();
        if input.is_key_pressed(Key::Enter) {
            // TODO:
        }
        if input.is_key_pressed(Key::Up) {
            // fb.select_prev();
            fb.select_prev_wrapped();
        }
        if input.is_key_pressed(Key::Down) {
            // fb.select_next();
            fb.select_next_wrapped();
        }
    }
}

/// A file list presenter.
#[derive(Debug)]
pub struct FileListPresenter {
    row_height: f32,
}

impl Default for FileListPresenter {
    fn default() -> Self {
        Self::new(32.0)
    }
}

impl FileListPresenter {
    /// Construct a new presenter for file list.
    pub fn new(row_height: f32) -> Self {
        Self { row_height }
    }

    fn scroll_to_selected(&self, ctx: &mut Context, fl: &FileList) {
        let mut y: i32 = 0;
        for (i, _f) in fl.iter().enumerate() {
            y += self.row_height as i32;
            if fl.selected == i {
                break;
            }
        }
        let win_size = ctx.window_get_size();
        let offset = y - win_size.y as i32 + (self.row_height * 2.0) as i32;
        if offset > 0 {
            ctx.window_set_scroll(0, offset as u32);
        } else {
            ctx.window_set_scroll(0, 0);
        }
    }

    /// Present each file item on the `ctx`.
    pub fn present(self, ctx: &mut Context, fl: &FileList) {
        // Save current window states
        let spacing = *ctx.style().window().spacing();
        let padding = *ctx.style().window().padding();
        // Remove spacing and padding
        ctx.style_mut().window_mut().set_spacing(vec2(0.0, 0.0));
        ctx.style_mut().window_mut().set_padding(vec2(0.0, 0.0));
        // Scroll to selected item if necessary
        self.scroll_to_selected(ctx, fl);
        let selected_bg_color = ctx.style().window().background().inverted();
        let selected_fg_color = ctx.style().text().color.inverted();
        // Render each file item
        for (i, f) in fl.iter().enumerate() {
            if fl.selected == i {
                ctx.layout_row_colored(
                    LayoutFormat::Dynamic,
                    self.row_height,
                    &[0.2, 0.4, 0.4],
                    selected_bg_color,
                );
                ctx.label_colored(
                    format!("{:-4}", i).into(),
                    FlagsBuilder::align().left().middle().into(),
                    selected_fg_color,
                );
                ctx.label_colored(
                    NkString::from(&f.file_name),
                    FlagsBuilder::align().left().middle().into(),
                    selected_fg_color,
                );
                ctx.label_colored(
                    NkString::from(
                        DateTime::<Local>::from(f.modified)
                            .format("%F %T")
                            .to_string(),
                    ),
                    FlagsBuilder::align().left().middle().into(),
                    selected_fg_color,
                );
            } else {
                ctx.layout_row(LayoutFormat::Dynamic, self.row_height, &[0.2, 0.4, 0.4]);
                ctx.label(
                    format!("{:-4}", i).into(),
                    FlagsBuilder::align().left().middle().into(),
                );
                ctx.label(
                    NkString::from(&f.file_name),
                    FlagsBuilder::align().left().middle().into(),
                );
                ctx.label(
                    NkString::from(
                        DateTime::<Local>::from(f.modified)
                            .format("%F %T")
                            .to_string(),
                    ),
                    FlagsBuilder::align().left().middle().into(),
                );
            }
        }
        // Restore old window states
        ctx.style_mut().window_mut().set_spacing(spacing);
        ctx.style_mut().window_mut().set_padding(padding);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_list() {
        let fb = FileList::new("./src", "rs");
        println!("{:#?}", fb);
    }
}
