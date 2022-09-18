extern crate core;

use std::{fs, path::PathBuf};

fn get_data(name: &str) -> (String, Vec<u8>) {
    let mut path = PathBuf::from("./tests/test-data/pairs/");
    path.push(name);

    path.set_extension("bin");
    let bin = fs::read(&path)
        .unwrap_or_else(|_| panic!("Failed to read binary data from {}", path.display()));

    path.set_extension("txt");
    let txt = fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Failed to read encoded data from {}", path.display()));

    (txt, bin)
}

macro_rules! test_data_pairs {
    ($($path:ident),+) => {$(
        #[allow(non_snake_case)]
        mod $path {
            const NAME: &str = stringify!($path);

            #[test]
            fn encode() {
                let (txt, bin) = super::get_data(&NAME);
                let enc = base2048::encode(&bin);

                assert_eq!(*txt, enc, "
The data in {}.bin was encoded wrongly:
Expected: {}
   Found: {}

Expected (last char): {}
   Found (last char): {}
",
                           NAME,
                           txt,
                           enc,
                           txt.chars().last().unwrap_or(' '),
                           enc.chars().last().unwrap_or(' '),
                );
            }

            #[test]
            fn decode() {
                let (txt, bin) = super::get_data(&NAME);
                let dec = base2048::decode(&txt)
                    .unwrap_or_else(|| panic!("Failed to decode the data in {}.bin", NAME));

                println!("Leftover/tail bits: {}", (bin.len() * 8) % 11);

                assert_eq!(bin, dec, "
The data in {}.txt was decoded wrongly:
Expected: {:?}

   Found: {:?}

Expected (last byte): {3:3} / {3:#08b}
   Found (last byte): {4:3} / {4:#08b}
",
                           NAME,
                           bin,
                           dec,
                           bin.last().unwrap_or(&0),
                           dec.last().unwrap_or(&0),
                );
            }
        }
    )+}
}

test_data_pairs!(
    case_demo,
    case_empty,
    every_byte,
    every_pair_of_bytes,
    hatetris_wr,
    hatetris_wr_rle,
    hatetris_wr_rle2,
    lena_std_tif
);