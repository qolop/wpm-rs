extern crate rand;
extern crate stopwatch;

use stopwatch::Stopwatch;
use std::io;
use rand::{thread_rng, Rng};

type Excerpt = String;

struct WPM {
    excerpt: Excerpt,
}

impl WPM {
    fn new() -> Self {
        let words = ["You know what I shall die of? I will die with my hand in the hand of some \
                      nice-looking ship's doctor, a very young one with a small blond moustache \
                      and a big silver watch. \"Poor lady\", they'll say, \"the quinine did her \
                      no good. That unwashed grape has transported her soul to heaven.\" And \
                      I'll be buried at sea, sewn up in a clean white sack, and dropped \
                      overboard at noon in the blaze of summer, and into an ocean as blue as my \
                      first lover's eyes!"];
        let mut rng = thread_rng();
        WPM { excerpt: rng.choose(&words).unwrap().to_string() }
    }

    fn run(&self) {
        println!("{}\n\n\n", self.excerpt);
        self.record();
    }

    fn record(&self) {

        let mut input = String::new();
        let sw = Stopwatch::start_new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let time = sw.elapsed_ms() as f64;
                // not very elegant but until I can find a better way of doing this
                let input_word_count = input.trim().split(" ").collect::<Vec<&str>>().len() as f64;
                println!("You typed the excerpt in took {:.2} seconds.",
                         time / 1000.0);
                let accuracy = self.get_accuracy(input.trim());
                println!("You typed {:.2} words per minute with {:.2}% accuracy.",
                         input_word_count / ((time / 1000.0) / 60.0),
                         accuracy);
            }
            Err(error) => println!("error: {}", error),
        }
    }

    fn get_accuracy(&self, s: &str) -> f64 {
        let mut accuracy = 0.0;
        let total_chars = self.excerpt.len() as f64;

        for (b, n) in s.chars().enumerate() {
            match self.excerpt.chars().nth(b) {
                Some(e) => {
                    if e == n {
                        accuracy += 1.0;
                    }
                }
                _ => {}
            }
        }

        accuracy / total_chars * 100.0
    }
}

fn main() {
    let wpm = WPM::new();
    wpm.run();
}
