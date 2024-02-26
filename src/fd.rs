use memfd;
use memmap::MmapOptions;

pub(crate) fn new_sized_memfd(size: u64) -> Result<memfd::Memfd, Box<dyn std::error::Error>> {
    // Create a sealable memfd.
    let opts = memfd::MemfdOptions::default().allow_sealing(true);
    let mfd = opts.create("sized-1K")?;

    // Resize to 1024B.
    mfd.as_file().set_len(size)?;

    // Add seals to prevent further resizing.
    mfd.add_seals(&[
        memfd::FileSeal::SealShrink,
        memfd::FileSeal::SealGrow
    ])?;

    // Prevent further sealing changes.
    mfd.add_seal(memfd::FileSeal::SealSeal)?;

    let _ = unsafe { MmapOptions::new().len(1024).map_mut(mfd.as_file())? };

    Ok(mfd)
}
