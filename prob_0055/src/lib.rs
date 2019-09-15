/*
Implement a URL shortener with the following methods:

    shorten(url), which shortens the url into a six-character alphanumeric string, such as zLg6wl.
    restore(short), which expands the shortened string into the original url. If no such shortened string exists, return null.

Hint: What if we enter the same URL twice?

Time to complete: 1h 20m (still can't get it worked out with proper string references)
*/

extern crate rand;

use std::collections::HashMap;
use rand::Rng;


pub struct Shortener {
    url_to_short: HashMap<String, String>,
    short_to_url: HashMap<String, String>,
}

impl Shortener {
    pub fn new() -> Shortener {
        Shortener {
            url_to_short: HashMap::new(),
            short_to_url: HashMap::new(),
        }
    }

    pub fn get_short(&mut self, url: &str) -> String {
        if self.url_to_short.contains_key(url) {
            self.url_to_short.get(url).unwrap().to_string()
        } else {
            let rc_short= self.generate_key();
            let rc_url= url.to_string();
            self.url_to_short.insert(rc_url.clone(), rc_short.clone());
            self.short_to_url.insert(rc_short.clone(), rc_url);
            rc_short
        }
    }

    pub fn get_long(&self, url: &str) -> Option<&str> {
        self.short_to_url.get(url).map(|v| &v[..])
    }

    fn generate_key(&self) -> String {
        let mut rnd = rand::thread_rng();
        let mut short = String::with_capacity(6);
        for _ in 0..6 {
            let letter = match rnd.gen_range(0, 26*2+10) {
                n @ 0..=26 => ('a' as u8 + n) as char,
                n @ 26..=52 => ('A' as u8 + n) as char,
                n => ('0' as u8 + n) as char,
            };
            short.push(letter)
        }

        if self.short_to_url.contains_key(&short) {
            self.generate_key()
        } else {
            short
        }
    }
}

#[cfg(test)]
mod tests {
    use Shortener;

    #[test]
    fn test_it() {
        let mut shortner = Shortener::new();
        let short = shortner.get_short("MyName");

        assert_eq!("MyName", shortner.get_long(&short[..]).unwrap());
    }

    #[test]
    fn test_same_entry() {
        let mut shortner = Shortener::new();
        let short1 = shortner.get_short("MyName");
        let short2 = shortner.get_short("MyName");

        assert_eq!(short1, short2);
    }
}