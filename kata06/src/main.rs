fn main() {
    let filepath = "/usr/share/dict/words";
    let anagrams = kata06::get_anagrams(filepath);
    for (_, words) in anagrams {
        println!("{}", words.join(" "));
    }
}
