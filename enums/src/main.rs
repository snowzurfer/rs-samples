// Enums in Rust can be either similar to C++1X-style enums, where you can
// access the enum fields using EnumName::VariantName, or can behave similarly
// to C++17's std::variant

// We declare an enum that defines different types of dialects for a language.
// Enums variants can hold values too! E.g. other enums, custom types or
// built-in types
enum Dialect {
    Northern,
    Central,
    Southern
}

// Here we see in action the ability of an enum variant to hold a value, and
// not all the variants need to hold one! This is one of the
// advantages over a struct:
// the type you are storing in the enum can vary
enum Language {
    // These are the languages I am most familiar with so I know that there are
    // differences from "north-to-south" in terms of dialects. Yes, I am
    // oversimplifying it :)
    Italian(Dialect),
    BritishEnglish,
    Spanish(u32),
    French,
    German { characteristics: String }
}

// We can pattern match to determine what type of Language was passed to the
// function and we can also recursively pattern-match.
// There are other syntaxes for pattern matching, and they are more coincise in
// some situations.
fn print_lang(lang: Language) {
    match lang {
        Language::Italian(dialect) => {
            let dialect_name = match dialect {
                Dialect::Northern => "Milanese",
                Dialect::Central => "Fiorentino",
                Dialect::Southern => "Foggiano"
            };
            println!("Italian, dialect: {}", dialect_name);
        },
        // The "{:?}" in the print!() macro call allows us to use the Debug
        // print trait which will format the output differently from how it
        // would be formatted had we used "{}". It it possible to implement the
        // Debug print trait for custom types by using the macro
        //  #[derive(Debug)]
        // on top of a type declaration.
        Language::Spanish(num_speakers) =>
            println!("Spanish, num. of speakers: {:?}", num_speakers),
        Language::German {characteristics} =>
            println!("German {}", characteristics),
        // Using "_" allows us to pattern match anything else not specified in
        // the list above
        //
        // Pattern matching in Rust needs to be exhaustive; that means we need
        // to "consume" all the possible variants and using _ allows us to do
        // so easily
        _ => println!("Some other language"),
    }
}

fn main() {
    // We declare some enum variables to show the difference between different
    // instantiatinos when we pattern-match them
    let central_italian = Language::Italian(Dialect::Central);
    let southern_italian = Language::Italian(Dialect::Southern);
    let northern_italian = Language::Italian(Dialect::Northern);
    let spanish = Language::Spanish(1000000); // Lots of spanish speakers!
    let german = Language::German {characteristics: "Is cool".to_owned()};
    let british = Language::BritishEnglish;
    let french = Language::French;

    // Now we can pattern-match them to see the results of our hard work and see
    // enums pattern matching at work by calling into the function we have
    // created
    print_lang(central_italian);
    print_lang(british);
    print_lang(southern_italian);
    print_lang(northern_italian);
    print_lang(spanish);
    print_lang(german);
    print_lang(french);
}
