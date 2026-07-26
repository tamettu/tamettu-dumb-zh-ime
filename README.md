# 🌸tamettu dumb zh ime🌸
A zero-learning-curve Chinese ***stroke*** IME built in Rust.( ͡° ͜ʖ ͡°)
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
* You cannot enter a digit directly next to another digit.  
  - The program will either ignore it or return an error message. (i forgot which one. u can test test it👍)  

* You can mistype some strokes using the special `F` key, but you cannot exceed the allowed tolerance.  
  - e.g. `我` <- `resarF9` (i will show what is the digit after F mean in bollow)  
  - e.g. `我` <- `rrresaF9` (this will not work because the word 我 only has 2 `r` strokes, but 3 were entered here)  


🎉That's All!🎉
# ✨features✨
* 🔀You don't need to follow any stroke order
* base on the vector of the strokes
* SLOW, but low candidate collisions + non-native-Chinese-speaker-friendly (America Ya! Hello Hello Hello~)
* There has 2 mode: non vague search and vague search
  - `non vague search`
  - it shows u all words that fully match ur input.
  - `vague search`
  - enter `F` in any place to use this mode
  - F1-9 mean how long does u want the aim text can larger than ur input. (just use 9 if u cant find a word what ever u try)
  - this mode allow u mistype one of each letter `w`, `q`, `t`, and `y`
* if a word have many of the same stroke such as `回` have 4 `w` and 4 `e`, u can type 4w4e, it will still give u the words u want.
* if a word have many same part such as the word `繼` have 5 same part(rwrwq), u can type 5RWRWQ(not full code), it will turn to 10r 10w 5q and go to find word
* if u only enter chinese_input, it will have a cli to make u enter text then it trow u back the words it found
* if u want to use wofi or other special UI app, this program support u enter text behide chinese_input (chinese_input xxxyyyzzz), then it will trow it out to std, no need to come in my the program!!
* dont watch my code pls. they are so beautiful that i scared u will stroke right after u see it.

🎉YAPPI!🎉

# ⚠️Footnotes⚠️
- the program and the words' data(only 1500 words so far) isnt done yet, but just like my other project, idk will i forget it tomorrow when i wake up, so dont have high expectations i will upgrade it...
- if u want to updata words, the web i follow is (here)[https://humanum.arts.cuhk.edu.hk/Lexis/lexi-can/faq.php?s=1001]. i will tank you so much if u send me the updated version!!!
- u can contant me by my discord link 
