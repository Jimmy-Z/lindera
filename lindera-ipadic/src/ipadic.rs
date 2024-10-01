#[cfg(feature = "ipadic")]
use std::env;

#[cfg(feature = "compress")]
use lindera_core::decompress::decompress;
use lindera_core::dictionary::character_definition::CharacterDefinition;
use lindera_core::dictionary::connection_cost_matrix::ConnectionCostMatrix;
use lindera_core::dictionary::prefix_dictionary::PrefixDictionary;
use lindera_core::dictionary::unknown_dictionary::UnknownDictionary;
use lindera_core::dictionary::Dictionary;
use lindera_core::LinderaResult;

macro_rules! decompress_data {
    ($name: ident, $bytes: expr, $filename: literal) => {
        #[cfg(feature = "compress")]
        static $name: once_cell::sync::Lazy<Vec<u8>> = once_cell::sync::Lazy::new(|| {
            let compressed_data = bincode::deserialize_from(&$bytes[..])
                .expect(concat!("invalid file format ", $filename));
            decompress(compressed_data).expect(concat!("invalid file format ", $filename))
        });
        #[cfg(not(feature = "compress"))]
        const $name: &'static [u8] = $bytes;
    };
}

#[cfg(feature = "ipadic")]
decompress_data!(
    CHAR_DEFINITION_DATA,
    include_bytes!(concat!(
        env!("LINDERA_WORKDIR"),
        "/lindera-ipadic/char_def.bin"
    )),
    "char_def.bin"
);
#[cfg(not(feature = "ipadic"))]
decompress_data!(CHAR_DEFINITION_DATA, &[], "char_def.bin");

#[cfg(feature = "ipadic")]
decompress_data!(
    CONNECTION_DATA,
    include_bytes!(concat!(
        env!("LINDERA_WORKDIR"),
        "/lindera-ipadic/matrix.mtx"
    )),
    "matrix.mtx"
);
#[cfg(not(feature = "ipadic"))]
decompress_data!(CONNECTION_DATA, &[], "matrix.mtx");

#[cfg(feature = "ipadic")]
decompress_data!(
    DA_DATA,
    include_bytes!(concat!(env!("LINDERA_WORKDIR"), "/lindera-ipadic/dict.da")),
    "dict.da"
);
#[cfg(not(feature = "ipadic"))]
decompress_data!(DA_DATA, &[], "dict.da");

#[cfg(feature = "ipadic")]
decompress_data!(
    VALS_DATA,
    include_bytes!(concat!(
        env!("LINDERA_WORKDIR"),
        "/lindera-ipadic/dict.vals"
    )),
    "dict.vals"
);
#[cfg(not(feature = "ipadic"))]
decompress_data!(VALS_DATA, &[], "dict.vals");

#[cfg(feature = "ipadic")]
decompress_data!(
    UNKNOWN_DATA,
    include_bytes!(concat!(env!("LINDERA_WORKDIR"), "/lindera-ipadic/unk.bin")),
    "unk.bin"
);
#[cfg(not(feature = "ipadic"))]
decompress_data!(UNKNOWN_DATA, &[], "unk.bin");

#[cfg(feature = "ipadic")]
decompress_data!(
    WORDS_IDX_DATA,
    include_bytes!(concat!(
        env!("LINDERA_WORKDIR"),
        "/lindera-ipadic/dict.wordsidx"
    )),
    "dict.wordsidx"
);
#[cfg(not(feature = "ipadic"))]
decompress_data!(WORDS_IDX_DATA, &[], "dict.wordsidx");

#[cfg(feature = "ipadic")]
decompress_data!(
    WORDS_DATA,
    include_bytes!(concat!(
        env!("LINDERA_WORKDIR"),
        "/lindera-ipadic/dict.words"
    )),
    "dict.words"
);
#[cfg(not(feature = "ipadic"))]
decompress_data!(WORDS_DATA, &[], "dict.words");

pub fn load() -> LinderaResult<Dictionary> {
    Ok(Dictionary {
        prefix_dictionary: PrefixDictionary::load(
            &DA_DATA,
            &VALS_DATA,
            &WORDS_IDX_DATA,
            &WORDS_DATA,
        ),
        connection_cost_matrix: ConnectionCostMatrix::load_static(&CONNECTION_DATA),
        character_definition: CharacterDefinition::load(&CHAR_DEFINITION_DATA)?,
        unknown_dictionary: UnknownDictionary::load(&UNKNOWN_DATA)?,
    })
}
