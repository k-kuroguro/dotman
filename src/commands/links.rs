use dialoguer::Confirm;
use simple_expand_tilde::expand_tilde;
use std::{
   fs::{create_dir_all, read_link, remove_file, symlink_metadata},
   os::unix::fs::symlink,
   path::Path,
};

use crate::config::Mapping;
use crate::error::Error;

pub fn install(
   mappings: &[Mapping],
   dotfiles_dir: &Path,
   force: bool,
   dry_run: bool,
) -> Result<(), Error> {
   for Mapping { src, dest } in mappings {
      let src = dotfiles_dir.join(src);
      let src = expand_tilde(&src).ok_or(Error::FailedToExpandTilde(src.clone()))?;
      let dest = expand_tilde(&dest).ok_or(Error::FailedToExpandTilde(dest.clone()))?;

      if !src.exists() {
         println!("Source file {} does not exist. Skipping.", src.display());
         continue;
      }

      if !dry_run {
         if dest.exists() {
            if !force {
               let confirmation = Confirm::new()
                  .with_prompt(&format!(
                     "Destination file {} already exists. Do you want to overwrite it?",
                     dest.display()
                  ))
                  .interact()
                  .map_err(|e| Error::Other(Box::new(e)))?;
               if !confirmation {
                  println!("Skipping {}", dest.display());
                  continue;
               }
            }
            remove_file(&dest).map_err(|e| Error::Other(Box::new(e)))?;
         }

         if let Some(parent) = dest.parent() {
            if !parent.exists() {
               create_dir_all(parent).map_err(|e| Error::Other(Box::new(e)))?;
            }
         }
         symlink(&src, &dest).map_err(|e| Error::Other(Box::new(e)))?;
      }

      println!("Link: {} -> {}", src.display(), dest.display());
   }
   Ok(())
}

pub fn remove(mappings: &[Mapping], dotfiles_dir: &Path) -> Result<(), Error> {
   for Mapping { src, dest } in mappings {
      let src = dotfiles_dir.join(src);
      let src = expand_tilde(&src).ok_or(Error::FailedToExpandTilde(src.clone()))?;
      let dest = expand_tilde(&dest).ok_or(Error::FailedToExpandTilde(dest.clone()))?;

      if is_actual_link(&src, &dest) {
         remove_file(&dest).map_err(|e| Error::Other(Box::new(e)))?;
         println!("Unlink: {}", dest.display());
      } else {
         println!(
            "{} does not symlink to {}. Skipping.",
            dest.display(),
            src.display(),
         );
      }
   }
   Ok(())
}

pub fn list(mappings: &[Mapping], dotfiles_dir: &Path) -> Result<(), Error> {
   for Mapping { src, dest } in mappings {
      let src = dotfiles_dir.join(src);
      let src = expand_tilde(&src).ok_or(Error::FailedToExpandTilde(src.clone()))?;
      let dest = expand_tilde(&dest).ok_or(Error::FailedToExpandTilde(dest.clone()))?;

      if is_actual_link(&src, &dest) {
         println!("Link: {} -> {}", src.display(), dest.display());
      }
   }
   Ok(())
}

fn is_actual_link(src: &Path, dest: &Path) -> bool {
   if let Ok(metadata) = symlink_metadata(dest) {
      if metadata.file_type().is_symlink() {
         if let Ok(link_target) = read_link(dest) {
            return link_target == src;
         }
      }
   }
   false
}
