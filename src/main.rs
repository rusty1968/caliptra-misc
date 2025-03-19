use der_tool::DerEccSignature;
use std::env;

fn main()  {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input_der_file> <output_file>", args[0]);
        return;
    }

    let input_file_path = &args[1];
    let output_file_path = &args[2];

    match DerEccSignature::from_der(input_file_path) {
        Some(signature) => {
            signature.to_file(output_file_path).unwrap();
            println!("Signature successfully read from {} and written to {}.", input_file_path, output_file_path);
        }
        None => {
            eprintln!("Failed to read the signature from the file.");
        }
    }    
}
