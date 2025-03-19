use caliptra_image_types::ImageEccSignature;
use der::{Decode, Sequence, SliceReader};
use std::io::{Read, Write};
use zerocopy::AsBytes;


#[derive(Sequence)]
pub struct DerEccSignature {
    r: der::asn1::Uint,
    s: der::asn1::Uint,
}

impl DerEccSignature {
    pub fn from_der(path: &str) -> Option<Self> {
        // Open the file
        let mut file = std::fs::File::open(path).ok()?;

        // Read the file contents into a buffer
        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer).ok()?;

        let mut reader = SliceReader::new(&buffer).ok()?;
        let signature = DerEccSignature::decode(&mut reader).ok()?;
        Some(signature)
    }
    pub fn to_le_words_12(bytes: &[u8]) -> anyhow::Result<[u32; 12]> {
        let mut out = [0u32; 12];
        if bytes.len() != std::mem::size_of_val(&out) {
            return Err(anyhow::Error::msg("ECC Integer has invalid size"));
        }

        for i in (0..bytes.len()).step_by(4) {
            out[i / 4] = u32::from_be_bytes(bytes[i..i + 4].try_into().unwrap());
        }

        Ok(out)
    }

    pub fn to_caliptra(&self) -> anyhow::Result<ImageEccSignature> {
        Ok(ImageEccSignature {
            r: Self::to_le_words_12(self.r.as_bytes())?,
            s: Self::to_le_words_12(self.s.as_bytes())?,
        })
    }

    pub fn to_file(&self, file_path: &str) -> anyhow::Result<()> {
        let mut file = std::fs::File::create(file_path)?;
        let sign = self.to_caliptra()?;
        file.write_all(sign.as_bytes())?;

        Ok(())
    }
}

#[test]
fn test_from_der() {
    let sign = DerEccSignature::from_der("test_data/sig.der").unwrap();   
    sign.to_file("caliptra_sign.bin").unwrap();
}

