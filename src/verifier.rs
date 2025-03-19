use std::fs::File;
use std::io::Read;

use caliptra_image_types::ImageManifest;
use openssl::sha::sha384;
use zerocopy::{AsBytes, FromBytes};

use caliptra_hw_model::{BootParams, Fuses, HwModel, InitParams, SecurityState};

pub struct FuseGenerator;

fn bytes_to_be_words_48(buf: &[u8; 48]) -> [u32; 12] {
    let mut result: [u32; 12] = zerocopy::transmute!(*buf);
    swap_word_bytes_inplace(&mut result);
    result
}
pub fn swap_word_bytes_inplace(words: &mut [u32]) {
    for word in words.iter_mut() {
        *word = word.swap_bytes()
    }
}

impl FuseGenerator {

    pub fn from_image(image: &[u8]) -> Fuses {

        let manifest = ImageManifest::read_from_prefix(image.as_bytes()).unwrap();

        let vendor_pk_hash = sha384(manifest.preamble.vendor_pub_keys.as_bytes());
        let owner_pk_hash = sha384(manifest.preamble.owner_pub_keys.as_bytes());
        let vendor_pk_hash_words = bytes_to_be_words_48(&vendor_pk_hash);
        let owner_pk_hash_words = bytes_to_be_words_48(&owner_pk_hash);

        caliptra_hw_model::Fuses {
            key_manifest_pk_hash: vendor_pk_hash_words,
            owner_pk_hash: owner_pk_hash_words,
            ..Default::default()
        }
    
    }

    
    
}



fn read_file_to_vec(file_path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

pub fn test_image(security_state: &SecurityState, fuses: &Fuses, fw_image: Vec<u8>) {
    const RT_READY_FOR_COMMANDS: u32 = 0x600;

    let rom = read_file_to_vec("test_data/caliptra-rom-with-log.bin").unwrap();
    let mut model = caliptra_hw_model::new(
        InitParams {
            rom: &rom,
            security_state: *security_state,
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
    use caliptra_hw_model::{DeviceLifecycle, SecurityState};

    use super::*;

    #[test]
    fn test_image_success() {

        let security_state =  *SecurityState::from(0)
        .set_debug_locked(false)
        .set_device_lifecycle(DeviceLifecycle::Production);


        // Create a sample firmware image
        let fw_image = include_bytes!("../test_data/image-bundle.bin");

        // Create a sample Fuses instance
        let fuses = FuseGenerator::from_image(fw_image);

        // Call the test_image function
        test_image(&security_state, &fuses, fw_image.to_vec());


        let fw_image = include_bytes!("../test_data/image-bundle_signed_with_testkey.bin");

        // Call the test_image function
        test_image(&security_state, &fuses, fw_image.to_vec());

        let fw_image = include_bytes!("../test_data/image-bundle_signed_with_newkey.bin");

        let fuses = FuseGenerator::from_image(fw_image);

        // Call the test_image function
        test_image(&security_state, &fuses, fw_image.to_vec());


    }
}
