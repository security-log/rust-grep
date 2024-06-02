fn main() {
    let term_search = "world";
    let quote = "Hello world!";

    for line in quote.lines() {
        if line.contains(term_search) {
            println!("{quote}\nHave the term {term_search}");
        } else {
            println!("Not found");
        }
    }
}
