# match-pinyin-with-hanzi

How can I check that a Chinese sentence written in Chinese characters (汉字) matches with the sentence in pinyin (拼音)? 
Well, first I have to parse the pinyin (which is not so easy), then I have to iterate over the Chinese characters... wait, 儿 might or might not stick to the previous syllable...

This crate resolves all that mess. With this crate, all you need is:

```rust
use match_pinyin_with_hanzi::match_pinyin_with_hanzi;
match_pinyin_with_hanzi(
    "māmā qí mǎ, mǎ màn, māma mà mǎ.", 
    "妈妈骑马，马慢，妈妈骂马。"
).unwrap();
```

Note that both `māmā` and `māma` are allowed to match with `妈妈`. This crate assumes that any syllable can lose its tone, so it is perfectly okay to match a toneless pinyin to the Chinese character.
