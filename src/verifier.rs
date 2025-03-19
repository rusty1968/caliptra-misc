use std::fs::File;
use std::io::Read;

use caliptra_hw_model::{BootParams, Fuses, HwModel, InitParams};

fn read_file_to_vec(file_path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

pub fn test_image(fuses: &Fuses, fw_image: Vec<u8>) {
    const RT_READY_FOR_COMMANDS: u32 = 0x600;

    let rom = read_file_to_vec("test_data/caliptra-rom-with-log.bin").unwrap();
    let mut model = caliptra_hw_model::new(
        InitParams {
            rom: &rom,
            ..Default::default()
        },
        BootParams {
            fuses: *fuses,
            fw_image: Some(&fw_image),
            ..Default::default()
        },
    )
    .unwrap();
    model.step_until_boot_status(RT_READY_FOR_COMMANDS, true);
}

// Tests module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_success() {
        // Create a sample Fuses instance
        let fuses = Fuses::default();

        // Create a sample firmware image
        let fw_image = include_bytes!("../test_data/image-bundle.bin");

        // Call the test_image function
        test_image(&fuses, fw_image.to_vec());
    }
}
