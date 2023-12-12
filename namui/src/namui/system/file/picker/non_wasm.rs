use crate::File;

/// NOTE: This would not emit any events if user cancels the file selection and closes the picker.
pub async fn open() -> Box<[File]> {
    rfd::AsyncFileDialog::new()
        .pick_files()
        .await
        .unwrap()
        .into_iter()
        .map(|file| File::new(file))
        .collect::<Vec<_>>()
        .into_boxed_slice()
}
