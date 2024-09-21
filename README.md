# Lindera

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) [![Crates.io](https://img.shields.io/crates/v/lindera.svg)](https://crates.io/crates/lindera)

A morphological analysis library in Rust. This project fork from [kuromoji-rs](https://github.com/fulmicoton/kuromoji-rs).

Lindera aims to build a library which is easy to install and provides concise APIs for various Rust applications.

The following products are required to build:

- Rust >= 1.46.0

## Tokenization examples

### Basic tokenization

Put the following in Cargo.toml:

```toml
[dependencies]
lindera = { version = "0.33.0", features = ["ipadic"] }
```

This example covers the basic usage of Lindera.

It will:

- Create a tokenizer in normal mode
- Tokenize the input text
- Output the tokens

```rust
use lindera::core::LinderaResult;
use lindera::core::mode::Mode;
use lindera::dictionary::{DictionaryConfig, DictionaryKind, DictionaryLoader};
use lindera::tokenizer::Tokenizer;

fn main() -> LinderaResult<()> {
    // Create a dictionary config.
    let dictionary_config = DictionaryConfig {
        kind: Some(DictionaryKind::IPADIC),
        path: None,
    };

    // Load a dictionary from the dictionary config.
    let dictionary = DictionaryLoader::load_dictionary_from_config(dictionary_config)?;

    // create tokenizer
    let tokenizer = Tokenizer::from_config(config)?;

    // Tokenize a text.
    let text = "関西国際空港限定トートバッグ";
    let mut tokens = tokenizer.tokenize(text)?;

    // Print the text and tokens.
    println!("text:\t{}", text);
    for token in tokens.iter_mut() {
        let details = token.details().join(",");
        println!("token:\t{}\t{}", token.text.as_ref(), details);
    }

    Ok(())
}
```

The above example can be run as follows:

```shell
% cargo run --features=ipadic --example=tokenize
```

You can see the result as follows:

```text
text:   関西国際空港限定トートバッグ
token:  関西国際空港    名詞,固有名詞,組織,*,*,*,関西国際空港,カンサイコクサイクウコウ,カンサイコクサイクーコー
token:  限定    名詞,サ変接続,*,*,*,*,限定,ゲンテイ,ゲンテイ
token:  トートバッグ    UNK
```

### Tokenization with user dictionary

You can give user dictionary entries along with the default system dictionary. User dictionary should be a CSV with following format.

```csv
<surface>,<part_of_speech>,<reading>
```

Put the following in Cargo.toml:

```toml
[dependencies]
lindera = { version = "0.33.0", features = ["ipadic"] }
```

For example:

```shell
% cat ./resources/simple_userdic.csv
東京スカイツリー,カスタム名詞,トウキョウスカイツリー
東武スカイツリーライン,カスタム名詞,トウブスカイツリーライン
とうきょうスカイツリー駅,カスタム名詞,トウキョウスカイツリーエキ
```

With an user dictionary, `Tokenizer` will be created as follows:

```rust
use std::path::PathBuf;

use lindera::core::LinderaResult;
use lindera::core::mode::Mode;
use lindera::dictionary::{
    DictionaryConfig, DictionaryKind, DictionaryLoader, UserDictionaryConfig,
};
use lindera::tokenizer::Tokenizer;

fn main() -> LinderaResult<()> {
    // Create a dictionary config.
    let dictionary_config = DictionaryConfig {
        kind: Some(DictionaryKind::IPADIC),
        path: None,
    };

    // Load a dictionary from the dictionary config.
    let dictionary = DictionaryLoader::load_dictionary_from_config(dictionary_config)?;

    let user_dictionary_config = UserDictionaryConfig {
        kind: Some(DictionaryKind::IPADIC),
        path: PathBuf::from("./resources/ipadic_simple_userdic.csv"),
    };

    let user_dictionary =
        DictionaryLoader::load_user_dictionary_from_config(user_dictionary_config)?;

    // Create a tokenizer.
    let tokenizer = Tokenizer::new(Mode::Normal, dictionary, Some(user_dictionary));

    // Tokenize a text.
    let text = "東京スカイツリーの最寄り駅はとうきょうスカイツリー駅です";
    let mut tokens = tokenizer.tokenize(text)?;

    // Print the text and tokens.
    println!("text:\t{}", text);
    for token in tokens.iter_mut() {
        let details = token.details().join(",");
        println!("token:\t{}\t{}", token.text.as_ref(), details);
    }

    Ok(())
}
```

The above example can be by `cargo run --example`:

```shell
% cargo run --features=ipadic --example=tokenize_with_user_dict
text:   東京スカイツリーの最寄り駅はとうきょうスカイツリー駅です
token:  東京スカイツリー        カスタム名詞,*,*,*,*,*,東京スカイツリー,トウキョウスカイツリー,*
token:  の      助詞,連体化,*,*,*,*,の,ノ,ノ
token:  最寄り駅        名詞,一般,*,*,*,*,最寄り駅,モヨリエキ,モヨリエキ
token:  は      助詞,係助詞,*,*,*,*,は,ハ,ワ
token:  とうきょうスカイツリー駅        カスタム名詞,*,*,*,*,*,とうきょうスカイツリー駅,トウキョウスカイツリーエキ,*
token:  です    助動詞,*,*,*,特殊・デス,基本形,です,デス,デス
```

### Tokenize with filters

Put the following in Cargo.toml:

```toml
[dependencies]
lindera = { version = "0.33.0", features = ["ipadic"] }
```

This example covers the basic usage of Lindera Analysis Framework.

It will:

- Apply character filter for Unicode normalization (NFKC)
- Tokenize the input text with IPADIC
- Apply token filters for removing stop tags (Part-of-speech) and Japanese Katakana stem filter

```rust
use std::collections::HashSet;

use lindera::character_filter::japanese_iteration_mark::{
    JapaneseIterationMarkCharacterFilter, JapaneseIterationMarkCharacterFilterConfig,
};
use lindera::character_filter::unicode_normalize::{
    UnicodeNormalizeCharacterFilter, UnicodeNormalizeCharacterFilterConfig,
    UnicodeNormalizeKind,
};
use lindera::character_filter::BoxCharacterFilter;
use lindera::core::LinderaResult;
use lindera::core::mode::Mode;
use lindera::dictionary::{DictionaryConfig, DictionaryKind, DictionaryLoader};
use lindera::token_filter::japanese_compound_word::{
    JapaneseCompoundWordTokenFilter, JapaneseCompoundWordTokenFilterConfig,
};
use lindera::token_filter::japanese_number::{
    JapaneseNumberTokenFilter, JapaneseNumberTokenFilterConfig,
};
use lindera::token_filter::japanese_stop_tags::{
    JapaneseStopTagsTokenFilter, JapaneseStopTagsTokenFilterConfig,
};
use lindera::token_filter::BoxTokenFilter;
use lindera::tokenizer::Tokenizer;

fn main() -> LinderaResult<()> {
    // Create a dictionary config.
    let dictionary_config = DictionaryConfig {
        kind: Some(DictionaryKind::IPADIC),
        path: None,
    };

    // Load a dictionary from the dictionary config.
    let dictionary = DictionaryLoader::load_dictionary_from_config(dictionary_config)?;

    // Create a tokenizer.
    let mut tokenizer = Tokenizer::new(Mode::Normal, dictionary, None);

    // Append a unicode normalize character filter to the tokenizer.
    let unicode_normalize_character_filter_config =
        UnicodeNormalizeCharacterFilterConfig::new(UnicodeNormalizeKind::NFKC);
    let unicode_normalize_character_filter =
        UnicodeNormalizeCharacterFilter::new(unicode_normalize_character_filter_config);
    tokenizer
        .append_character_filter(BoxCharacterFilter::from(unicode_normalize_character_filter));

    // Append a japanese iteration mark character filter to the tokenizer.
    let japanese_iteration_mark_character_filter_config =
        JapaneseIterationMarkCharacterFilterConfig::new(true, true);
    let japanese_iteration_mark_character_filter = JapaneseIterationMarkCharacterFilter::new(
        japanese_iteration_mark_character_filter_config,
    );
    tokenizer.append_character_filter(BoxCharacterFilter::from(
        japanese_iteration_mark_character_filter,
    ));

    // Append a japanese compound word token filter to the tokenizer.
    let japanese_compound_word_token_filter_config =
        JapaneseCompoundWordTokenFilterConfig::new(
            DictionaryKind::IPADIC,
            HashSet::from_iter(vec!["名詞,数".to_string()]),
            Some("名詞,数".to_string()),
        )?;
    let japanese_compound_word_token_filter =
        JapaneseCompoundWordTokenFilter::new(japanese_compound_word_token_filter_config);
    tokenizer.append_token_filter(BoxTokenFilter::from(japanese_compound_word_token_filter));

    // Append a japanese number token filter to the tokenizer.
    let japanese_number_token_filter_config =
        JapaneseNumberTokenFilterConfig::new(Some(HashSet::from_iter(vec![
            "名詞,数".to_string()
        ])));
    let japanese_number_token_filter =
        JapaneseNumberTokenFilter::new(japanese_number_token_filter_config);
    tokenizer.append_token_filter(BoxTokenFilter::from(japanese_number_token_filter));

    // Append a japanese stop tags token filter to the tokenizer.
    let japanese_stop_tags_token_filter_config =
        JapaneseStopTagsTokenFilterConfig::new(HashSet::from_iter(vec![
            "接続詞".to_string(),
            "助詞".to_string(),
            "助詞,格助詞".to_string(),
            "助詞,格助詞,一般".to_string(),
            "助詞,格助詞,引用".to_string(),
            "助詞,格助詞,連語".to_string(),
            "助詞,係助詞".to_string(),
            "助詞,副助詞".to_string(),
            "助詞,間投助詞".to_string(),
            "助詞,並立助詞".to_string(),
            "助詞,終助詞".to_string(),
            "助詞,副助詞／並立助詞／終助詞".to_string(),
            "助詞,連体化".to_string(),
            "助詞,副詞化".to_string(),
            "助詞,特殊".to_string(),
            "助動詞".to_string(),
            "記号".to_string(),
            "記号,一般".to_string(),
            "記号,読点".to_string(),
            "記号,句点".to_string(),
            "記号,空白".to_string(),
            "記号,括弧閉".to_string(),
            "その他,間投".to_string(),
            "フィラー".to_string(),
            "非言語音".to_string(),
        ]));
    let japanese_stop_tags_token_filter =
        JapaneseStopTagsTokenFilter::new(japanese_stop_tags_token_filter_config);
    tokenizer.append_token_filter(BoxTokenFilter::from(japanese_stop_tags_token_filter));

    // Tokenize a text.
    let text = "Ｌｉｎｄｅｒａは形態素解析ｴﾝｼﾞﾝです。ユーザー辞書も利用可能です。";
    let tokens = tokenizer.tokenize(text)?;

    // Print the text and tokens.
    println!("text: {}", text);
    for token in tokens {
        println!(
            "token: {:?}, start: {:?}, end: {:?}, details: {:?}",
            token.text, token.byte_start, token.byte_end, token.details
        );
    }

    Ok(())
}
```

The above example can be run as follows:

```shell
% cargo run --features=ipadic --example=tokenize_with_filters
```

You can see the result as follows:

```text
text: Ｌｉｎｄｅｒａは形態素解析ｴﾝｼﾞﾝです。ユーザー辞書も利用可能です。
token: "Lindera", start: 0, end: 21, details: Some(["UNK"])
token: "形態素", start: 24, end: 33, details: Some(["名詞", "一般", "*", "*", "*", "*", "形態素", "ケイタイソ", "ケイタイソ"])
token: "解析", start: 33, end: 39, details: Some(["名詞", "サ変接続", "*", "*", "*", "*", "解析", "カイセキ", "カイセキ"])
token: "エンジン", start: 39, end: 54, details: Some(["名詞", "一般", "*", "*", "*", "*", "エンジン", "エンジン", "エンジン"])
token: "ユーザー", start: 63, end: 75, details: Some(["名詞", "一般", "*", "*", "*", "*", "ユーザー", "ユーザー", "ユーザー"])
token: "辞書", start: 75, end: 81, details: Some(["名詞", "一般", "*", "*", "*", "*", "辞書", "ジショ", "ジショ"])
token: "利用", start: 84, end: 90, details: Some(["名詞", "サ変接続", "*", "*", "*", "*", "利用", "リヨウ", "リヨー"])
token: "可能", start: 90, end: 96, details: Some(["名詞", "形容動詞語幹", "*", "*", "*", "*", "可能", "カノウ", "カノー"])
```

## API reference

The API reference is available. Please see following URL:

- [lindera](https://docs.rs/lindera)
