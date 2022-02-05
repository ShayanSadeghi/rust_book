struct ImportantEx<'a> {
    part: &'a str,
}

impl<'a> ImportantEx<'a> {
    fn level(&self) -> i32 {
        // the return value is not reference to anything, so we don't need to define lifetime
        3
    }
}

impl<'a> ImportantEx<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        // lifetime elision rule: because one of parameters is &self, the return type gets the
        // ...lifetime of &self.
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let string1 = String::from("abcd");
    {
        let string2 = "xyz";
        let result = longest(string1.as_str(), string2);
        // if we try to print result out of this inner scope,
        // compiler shows an error because the lifetime of string2 is not
        // longer than this scope and result lifetime is equal to
        // shorter lifetime of string1 and string 2
        println!("The longest string is \"{}\"", result);
    }

    let dum = dummy_func("hi", "hello");
    println!("dummy variable is {}", dum);

    let novel = String::from("It is a test string. it is another sentence...");
    let first_sentence = novel.split('.').next().expect("could not find a '.'");
    let x = ImportantEx {
        part: first_sentence,
    };
    println!("x part is {}", x.part);

    println!("method level returns:\t {}", x.level());
    x.announce_and_return_part("Check amper");
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // lifetime names started with an apostrophe
    // this code means the lifetime of the reference returned by "longest" function is the same as
    // ...the smaller of the liftetimes of the refrences passed in
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn dummy_func<'a>(x: &'a str, y: &str) -> &'a str {
    println!("second parameter is: {}", y);
    // a dummy function like "longest" but only return the first parameter passed to it,
    // notice we only use lifetime "'a" for x
    x // simply return the first parameter
}
