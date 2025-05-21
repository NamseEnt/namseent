use super::{Path, PathElement};
use crate::*;
use std::{fs::DirEntry, path::PathBuf};

#[derive(PartialEq, Eq, Debug)]
pub struct IncludeOperation {
    src_path: Path,
    dest_path: Path,
}
impl IncludeOperation {
    pub fn new(src_path: Path, dest_path: Path) -> Self {
        Self {
            src_path,
            dest_path,
        }
    }
    pub fn join_dest_path_under_dest_root_path(
        &self,
        dest_root_path: impl AsRef<std::path::Path>,
    ) -> Result<PathBuf> {
        let mut target_dest_path = dest_root_path.as_ref().to_path_buf();
        for element in &self.dest_path.elements {
            match element {
                PathElement::FileOrDir {
                    raw_string,
                    regex: _,
                } => target_dest_path.push(raw_string),
                PathElement::DoubleAsterisk => {
                    return Err(anyhow!(
                        "join_dest_path_under_dest_root_path: No wildcard allowed to dest_path"
                    ));
                }
                PathElement::CurrentDirectory => continue,
                PathElement::ParentDirectory => {
                    target_dest_path.pop();
                }
            }
        }
        Ok(target_dest_path)
    }

    pub fn visit<F>(
        &self,
        target_src_path: impl AsRef<std::path::Path>,
        target_dest_path: impl AsRef<std::path::Path>,
        src_path_depth: usize,
        keep_directory_structure: bool,
        op: &mut F,
    ) -> Result<()>
    where
        F: FnMut(PathBuf, PathBuf),
    {
        let target_src_path = target_src_path.as_ref();
        let target_dest_path = target_dest_path.as_ref();
        if !target_src_path.exists() {
            return Ok(());
        }
        match self.src_path.elements.get(src_path_depth) {
            Some(path_element) => match path_element {
                PathElement::FileOrDir {
                    raw_string: _,
                    regex,
                } => {
                    visit_just_under_directory(target_src_path, &mut |dirent| {
                        if !regex.is_match(dirent.file_name().to_str().unwrap()) {
                            return Ok(());
                        }
                        let target_dest_path =
                            match keep_directory_structure && dirent.path().is_dir() {
                                true => target_dest_path.join(dirent.file_name()),
                                false => target_dest_path.to_path_buf(),
                            };
                        self.visit(
                            dirent.path(),
                            target_dest_path,
                            src_path_depth + 1,
                            keep_directory_structure,
                            op,
                        )
                    })?;
                }
                PathElement::DoubleAsterisk => {
                    self.visit(
                        target_src_path,
                        target_dest_path,
                        src_path_depth + 1,
                        true,
                        op,
                    )?;
                    visit_just_under_directory(target_src_path, &mut |dirent| {
                        let target_dest_path = match dirent.path().is_dir() {
                            true => target_dest_path.join(dirent.file_name()),
                            false => target_dest_path.to_path_buf(),
                        };
                        self.visit(dirent.path(), target_dest_path, src_path_depth, true, op)
                    })?;
                }
                PathElement::CurrentDirectory => self.visit(
                    target_src_path,
                    target_dest_path,
                    src_path_depth + 1,
                    keep_directory_structure,
                    op,
                )?,
                PathElement::ParentDirectory => {
                    let mut target_src_path = target_src_path.to_path_buf();
                    target_src_path.pop();
                    let mut target_dest_path = target_dest_path.to_path_buf();
                    if keep_directory_structure {
                        target_dest_path.pop();
                    }

                    self.visit(
                        &target_src_path,
                        &target_dest_path,
                        src_path_depth + 1,
                        keep_directory_structure,
                        op,
                    )?
                }
            },
            None => match target_src_path.is_dir() {
                true => {
                    visit_just_under_directory(target_src_path, &mut |dirent| {
                        let target_dest_path = match dirent.path().is_dir() {
                            true => target_dest_path.join(dirent.file_name()),
                            false => target_dest_path.to_path_buf(),
                        };
                        self.visit(dirent.path(), target_dest_path, src_path_depth, true, op)
                    })?;
                }
                false => op(
                    target_src_path.to_path_buf(),
                    target_dest_path.to_path_buf(),
                ),
            },
        }
        Ok(())
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct ExcludeOperation {
    path: Path,
}
impl ExcludeOperation {
    pub fn new(path: Path) -> Self {
        Self { path }
    }

    pub fn visit<F>(
        &self,
        target_src_path: impl AsRef<std::path::Path>,
        src_path_depth: usize,
        op: &mut F,
    ) -> Result<()>
    where
        F: FnMut(PathBuf),
    {
        let target_src_path = target_src_path.as_ref();
        if !target_src_path.exists() {
            return Ok(());
        }
        match self.path.elements.get(src_path_depth) {
            Some(path_element) => match path_element {
                PathElement::FileOrDir {
                    raw_string: _,
                    regex,
                } => {
                    visit_just_under_directory(target_src_path, &mut |dirent| {
                        if !regex.is_match(dirent.file_name().to_str().unwrap()) {
                            return Ok(());
                        }
                        self.visit(dirent.path(), src_path_depth + 1, op)
                    })?;
                }
                PathElement::DoubleAsterisk => {
                    self.visit(target_src_path, src_path_depth + 1, op)?;
                    visit_just_under_directory(target_src_path, &mut |dirent| {
                        self.visit(dirent.path(), src_path_depth, op)
                    })?;
                }
                PathElement::CurrentDirectory => {
                    self.visit(target_src_path, src_path_depth + 1, op)?
                }
                PathElement::ParentDirectory => {
                    let mut target_src_path = target_src_path.to_path_buf();
                    target_src_path.pop();
                    self.visit(&target_src_path, src_path_depth + 1, op)?
                }
            },
            None => match target_src_path.is_dir() {
                true => {
                    visit_just_under_directory(target_src_path, &mut |dirent| {
                        self.visit(dirent.path(), src_path_depth, op)
                    })?;
                }
                false => op(target_src_path.to_path_buf()),
            },
        }
        Ok(())
    }
}

fn visit_just_under_directory<F>(directory: impl AsRef<std::path::Path>, op: &mut F) -> Result<()>
where
    F: FnMut(DirEntry) -> Result<()>,
{
    let directory = directory.as_ref();
    if !directory.is_dir() {
        return Ok(());
    }
    for dirent in directory
        .read_dir()
        .map_err(|error| anyhow!("error while read dir {:?}:\n\t{}", directory, error))?
    {
        let dirent = dirent
            .map_err(|error| anyhow!("error while read dirent {:?}:\n\t{}", directory, error))?;
        op(dirent)?;
    }
    Ok(())
}
