#[cfg(test)]
mod tests {
    use crate::match_pinyin_with_hanzi;
    #[test]
    fn it_works() {
        match_pinyin_with_hanzi("Nǐ qù nǎli?", "你去哪里？").unwrap();
    }
}

/// ```
/// use match_pinyin_with_hanzi::match_pinyin_with_hanzi;
/// match_pinyin_with_hanzi("māmā qí mǎ, mǎ màn, māma mà mǎ.", "妈妈骑马，马慢，妈妈骂马。").unwrap();
/// 
/// // Erhua is also supported.
/// // This sample sentence is taken from Wiktionary: https://en.wiktionary.org/w/index.php?title=%E4%B8%80%E9%BB%9E%E5%85%92&oldid=60782800
/// match_pinyin_with_hanzi("Jiù wèi zhème yīdiǎnr shìr shēngqì, zhídàng de ma?", "就為這麼一點兒事兒生氣，值當的嗎？").unwrap();
/// ```
pub fn match_pinyin_with_hanzi(pinyin_str: &str, hanzi_str: &str) -> Result<(), String> {
    use pinyin::ToPinyinMulti;
    use pinyin_parser::PinyinParser;

    let mut hanzi_iter = hanzi_str.chars();
    for pinyin in PinyinParser::strict(pinyin_str) {
        let (hanzi, candidates) = loop {
            let hanzi = hanzi_iter.next().expect("hanzi ran out");

            if let Some(multi) = hanzi.to_pinyin_multi() {
                let mut candidates = vec![];
                for cand_pinyin in multi {
                    candidates.push(cand_pinyin.with_tone());
                    candidates.push(cand_pinyin.plain()); // to allow light tone
                }
                break (hanzi, candidates);
            }
        };

        if pinyin.ends_with('r') && !["er", "ēr", "ér", "ěr", "èr"].contains(&&pinyin[..]) {
            // Erhua. Get the next Chinese character and verify that it is 儿 or 兒
            loop {
                let expect_儿 = hanzi_iter.next().expect("hanzi ran out, expected 儿 or 兒");
                if expect_儿.to_pinyin_multi().is_some() {
                    if "儿兒".contains(expect_儿) {
                        break;
                    }
                    return Err(format!(
                        "expected 儿 or 兒 because of the rhotic pinyin {}, but instead found a Chinese character {}",
                        pinyin, expect_儿
                    ));
                }
            }
        } else {
            if candidates.contains(&&pinyin[..]) {
                continue;
            }

            return Err(format!(
                "{} not found within candidates {:?} possible for the Chinese character {}. Encountered this while matching `{}` with `{}`.",
                pinyin, candidates, hanzi, pinyin_str, hanzi_str
            ));
        }
    }

    Ok(())
}
