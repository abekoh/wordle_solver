use crate::{Hint, Solver, Spot};

#[derive(Debug)]
pub struct SimpleSolver {
    width: usize,
    dict_words: Vec<String>,
}

impl SimpleSolver {
    pub fn new(width: usize, dict_words: &Vec<String>) -> SimpleSolver {
        SimpleSolver {
            width,
            dict_words: dict_words.iter()
                .filter(|word| {
                    word.len() == width
                })
                .map(|word| {
                    word.to_string()
                }).collect(),
        }
    }

    fn shrink_hints(hints: &[Hint]) -> Vec<Hint> {
        let mut results: Vec<Hint> = Vec::new();
        for hint in hints {
            if hint.spot == Spot::None()
                && hints
                .iter()
                .filter(|h| {
                    match h.spot {
                        Spot::At(_) => h.letter == hint.letter,
                        _ => false,
                    }
                }).count() > 0 {
                continue;
            }
            results.push(hint.clone());
        }
        return results;
    }

    fn update_with_hints(&mut self, hints: &[Hint]) {
        self.dict_words = self.dict_words.iter()
            .filter(|word| {
                for hint in Self::shrink_hints(hints) {
                    let res = match &hint.spot {
                        Spot::None() => {
                            !word.contains(hint.letter)
                        }
                        Spot::InWithout(spot) => {
                            if !word.contains(hint.letter) {
                                return false;
                            }
                            word.as_bytes()[*spot as usize] as char != hint.letter
                        }
                        Spot::At(at_spot) => {
                            word.as_bytes()[(*at_spot)] as char == hint.letter
                        }
                    };
                    if !res {
                        return false;
                    }
                }
                true
            }).cloned()
            .collect();
    }

    fn remove_word(&mut self, word: &str) {
        if word.len() != self.width {
            return;
        }
        match self.dict_words.iter().position(|r| { r == word }) {
            Some(index) => {
                self.dict_words.swap_remove(index);
            }
            _ => (),
        }
    }
}

impl Solver for SimpleSolver {
    fn suggest(&self) -> &Vec<String> {
        &self.dict_words
    }

    fn add_hint(&mut self, word: &str, hints: &[Hint]) {
        self.remove_word(word);
        self.update_with_hints(hints);
    }

    fn remained_words_length(&self) -> usize {
        self.dict_words.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[cfg(test)]
    mod new {
        use super::*;

        #[test]
        fn asset_fields() {
            let target = SimpleSolver::new(5, &vec!["hello".to_string(), "early".to_string()]);
            assert_eq!(target.width, 5);
            assert_eq!(target.dict_words, vec!["hello", "early"]);
        }

        #[test]
        fn filter_word_only_length_is_5() {
            let target = SimpleSolver::new(5, &vec!["hello".to_string(), "dog".to_string(), "in".to_string(), "early".to_string(), "difference".to_string()]);
            assert_eq!(target.dict_words, vec!["hello", "early"]);
        }
    }

    #[test]
    fn shrink_words() {
        let actual = SimpleSolver::shrink_hints(&vec![
            Hint { letter: 'r', spot: Spot::None() },
            Hint { letter: 'o', spot: Spot::At(1) },
            Hint { letter: 'b', spot: Spot::None() },
            Hint { letter: 'o', spot: Spot::None() },
            Hint { letter: 't', spot: Spot::At(4) },
        ]);
        assert_eq!(actual, vec![
            Hint { letter: 'r', spot: Spot::None() },
            Hint { letter: 'o', spot: Spot::At(1) },
            Hint { letter: 'b', spot: Spot::None() },
            Hint { letter: 't', spot: Spot::At(4) },
        ])
    }

    #[cfg(test)]
    mod suggest {
        use super::*;

        fn preset_words() -> Vec<String> {
            vec!["hello".to_string(), "early".to_string(), "asset".to_string()]
        }


        #[test]
        fn remove_word() {
            let mut target = SimpleSolver::new(5, &preset_words());
            target.add_hint("hello", &vec![]);
            assert_eq!(target.suggest(), &vec![String::from("asset"), String::from("early")]);
        }

        #[cfg(test)]
        mod remove_including {
            use super::*;

            #[test]
            fn a() {
                let mut target = SimpleSolver::new(5, &preset_words());
                target.add_hint("dummy", &vec![Hint { letter: 'a', spot: Spot::None() }]);
                assert_eq!(target.suggest(), &vec![String::from("hello")]);
            }

            #[test]
            fn l() {
                let mut target = SimpleSolver::new(5, &preset_words());
                target.add_hint("dummy", &vec![Hint { letter: 'l', spot: Spot::None() }]);
                assert_eq!(target.suggest(), &vec![String::from("asset")]);
            }

            #[test]
            fn none_but_include() {
                let mut actual = SimpleSolver::new(
                    5,
                    &vec!["early".to_string()],
                );
                actual.add_hint("robot", &vec![
                    Hint { letter: 's', spot: Spot::None() },
                    Hint { letter: 'k', spot: Spot::None() },
                    Hint { letter: 'i', spot: Spot::None() },
                    Hint { letter: 'l', spot: Spot::At(3) },
                    Hint { letter: 'l', spot: Spot::None() },
                ]);
                assert_eq!(actual.suggest(), &vec![String::from("early")]);
            }
        }

        #[cfg(test)]
        mod only_including {
            use super::*;

            #[test]
            fn l_2() {
                let mut target = SimpleSolver::new(5, &preset_words());
                target.add_hint("dummy", &vec![Hint { letter: 'l', spot: Spot::InWithout(2) }]);
                assert_eq!(target.suggest(), &vec![String::from("early")]);
            }

            #[test]
            fn e_0() {
                let mut target = SimpleSolver::new(5, &preset_words());
                target.add_hint("dummy", &vec![Hint { letter: 'e', spot: Spot::InWithout(0) }]);
                assert_eq!(target.suggest(), &vec![String::from("hello"), String::from("asset")]);
            }
        }


        #[cfg(test)]
        mod at {
            use super::*;

            #[test]
            fn t_4() {
                let mut target = SimpleSolver::new(5, &preset_words());
                target.add_hint("dummy", &vec![Hint { letter: 't', spot: Spot::At(4) }]);
                assert_eq!(target.suggest(), &vec![String::from("asset")]);
            }
        }

        #[cfg(test)]
        mod multiple {
            use super::*;

            #[test]
            fn multiple_1() {
                let mut target = SimpleSolver::new(5, &vec![
                    "hello".to_string(),
                    "early".to_string(),
                    "asset".to_string(),
                    "bound".to_string(),
                    "heard".to_string(),
                    "spice".to_string(),
                ]);
                target.add_hint("bound", &vec![
                    Hint::new('b', Spot::None()),
                    Hint::new('o', Spot::None()),
                    Hint::new('u', Spot::None()),
                    Hint::new('n', Spot::None()),
                    Hint::new('d', Spot::None()),
                ]);
                assert_eq!(target.suggest(), &vec![String::from("early"), String::from("asset"), String::from("spice")]);
                target.add_hint("spice", &vec![
                    Hint::new('s', Spot::InWithout(0)),
                    Hint::new('p', Spot::None()),
                    Hint::new('i', Spot::None()),
                    Hint::new('c', Spot::None()),
                    Hint::new('e', Spot::InWithout(4)),
                ]);
                assert_eq!(target.suggest(), &vec![String::from("asset")]);
            }
        }
    }
}
