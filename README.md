# 🌸tamettu dumb zh ime🌸
A zero-learning-curve Chinese ***stroke*** IME built in Rust.
# 📋only 3 rules📋
* en -> zh strokes
  - `q` -> `丶` (top left to bottom right, no radius)
  - `w` -> `一` (any horizontal line)
  - `e` -> `丨` (any vertical line)
  - `r` -> `丿` (all lines from top right to bottom left, radius or not)
  - `t` -> `㇏` (top left to bottom right, with radius)
  - `y` -> `㇀` (bottom left to top right)
  - `s` -> `亅` (tick to the left, only tick count)
  - `a` -> `㇟` (tick to the right, only tick count)
  - |e.g.| `我` <- `rwesytarq`
  - |e.g.| `你` <- `rerwresqr`
  - |e.g.| `教` <- `wewrwreswrwrt`
* You cannot enter a digit directly next to another digit.  
  - The program will either ignore it or return an error message. (I forgot which one—you can test test!👍)  

* You can mistype some strokes using the special `F` key, but you cannot exceed the allowed tolerance.  
  - e.g. `我` <- `resarF9` (I will show what the digit after F means below)  
  - e.g. `我` <- `rrresaF9` (this will not work because the character 我 only has 2 `r` strokes, but 3 were entered here)  


🎉That's All!🎉
# ✨features✨
* 🔀You don't need to follow any stroke order
* Based on the vector of the strokes
* SLOW, but low candidate collisions + non-native-Chinese-speaker-friendly (America Ya! Hello Hello Hello~)
* There are 2 modes: non-vague search and vague search
  - `non-vague search`
  - Shows you all words that fully match your input.
  - `vague search`
  - Enter `F` in any place to use this mode
  - F1-9 means how much longer the target text can be compared to your input. (Just use 9 if you can't find a word whatever you try)
  - This mode allows you to mistype one of each letter: `w`, `q`, `t`, and `y`
* If a word has many of the same stroke, such as `回` having 4 `w`s and 4 `e`s, you can type `4w4e` and it will still give you the words you want.
* If a word has many of the same part, such as the word `繼` having 5 of the same part (rwrwq), you can type `5RWRWQ` (not full code); it will turn into `10r 10w 5q` and go find the word.
* If you only enter `chinese_input`, it will launch a CLI to let you enter text and then throw back the words it found.
* If you want to use wofi or another special UI app, this program supports entering text behind `chinese_input` (`chinese_input xxxyyyzzz`). It will then throw it out to stdout—no need to enter the interactive program!
* Don't look at my code, please. It is so beautiful that I'm scared you will stroke right after you see it.

🎉YAPPI!🎉

# ⚠️Footnotes⚠️
- The program and the word data (only 1,500 words so far) aren't done yet, but just like my other projects, I don't know if I'll forget about it tomorrow when I wake up, so don't have high expectations that I will update it...
- If you want to update words, the website I follow is [here](https://humanum.arts.cuhk.edu.hk/Lexis/lexi-can/faq.php?s=1001). I will thank you so much if you send me the updated version!!!
- Welcome to make any comments in my Discord server; I need your power to make this program better!!!
Check check the whole readme
