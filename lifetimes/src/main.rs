// NOTE: It's possible to assign generic lifetimes in structs as well
// In this case, we defined a `'a` generic lifetime on both the `title`, and `author`
// (the `T: 'a`) syntax means that the if the generic type holds a reference,
// then that reference MUST outlive `'a` (means that it MUST live at least as long as `'a`)
#[derive(Debug)]
struct Book<'a, T: 'a> {
    title: &'a str,
    author: T
}

fn main() {
    let str1 = String::from("Hello, world!");
    let result;
    {
        let str2 = String::from("Hello, world 2!");
        // NOTE: `result` valid here
        result = longest(&str2, &str1);
        println!("{result}");
    }
    // NOTE: `str2` goes out of scope so `result` will be invalidated as well
    // (possible returned pointer to be `str2` which is a dangling reference)
    // println!("{result}");

    let title =  "Book title";
    let new_book: Book<&str>;

    {
        let author = String::from("Feis ._.");

        // NOTE: This won't work because `author` does not live long enough
        // new_book = Book {
        //     title,
        //     author: &author
        // }
    }

    let author = String::from("Feis._.");
    // NOTE: This works instead because the lifetime of `author` is the same as `title`
    new_book = Book {
        title,
        author: &author
    };
    println!("{new_book:?}");
}

// NOTE: Defining a generic lifetime. This serves as a lifetime relationship
// annotation to be sure that the compiler will not return
// a possible dangling reference.
// In this case we are telling the compiler that `x` will live as long as `y` is valid
// and viceversa. Without the generic definition, the compiler would not be
// able to validate the lifetime relationship between the parameters and the returned value
// (since the method can be called from different places
// and parameters have different lifetimes between calls)
// NOTE: In this case the returned value will be valid as long as `x` and `y` are BOTH valid
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
