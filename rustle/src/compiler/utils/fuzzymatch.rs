use core::cmp::Ordering::Equal;
use std::collections::HashMap;

use regex::Regex;

pub fn fuzzymatch(name: &str, names: Vec<&str>) -> Option<String> {
    let set = FuzzySet::new(names);
    if let Some(matches) = set.get(&name) {
        if let Some(m) = matches.get(0) {
            let s = (m.1).clone();
            if m.0 > 0.7 {
                return Some(s);
            }
        };
    };

    return None;
}

const GRAM_SIZE_LOWER: usize = 2;
const GRAM_SIZE_UPPER: usize = 3;

// return an edit distance from 0 to 1
pub fn distance(str1: Option<&str>, str2: Option<&str>) -> Result<f32, String> {
    match (str1, str2) {
        (None, None) => Err(String::from("Trying to compare two null values")),
        (Some(_), None) | (None, Some(_)) => Ok(0.0),
        (Some(s1), Some(s2)) => Ok(get_distance(s1, s2)),
    }
}

pub fn get_distance(str1: &str, str2: &str) -> f32 {
    let distance = levenshtein(str1, str2);
    if str1.len() > str2.len() {
        return 1.0 - distance as f32 / str1.len() as f32;
    } else {
        return 1.0 - distance as f32 / str2.len() as f32;
    }
}

// helper functions
pub fn levenshtein(str1: &str, str2: &str) -> usize {
    let mut current: Vec<usize> = Vec::new();
    let mut prev: Option<usize> = None;
    let mut value: Option<usize>;

    for i in 0..=str2.len() {
        for j in 0..=str1.len() {
            if i > 0 && j > 0 {
                if str1.chars().nth(j - 1).unwrap_or(' ') == str2.chars().nth(i - 1).unwrap_or(' ')
                {
                    value = prev;
                } else {
                    value = Some(current[j].min(current[j - 1]).min(prev.unwrap()) + 1);
                }
            } else {
                value = Some(i + j);
            }

            prev = current.get(j).cloned();
            if current.len() > j {
                current[j] = value.unwrap();
            } else {
                current.push(value.unwrap());
            }
        }
    }
    current.pop().unwrap()
}

pub fn iterate_grams(value: &str, gram_size: Option<usize>) -> Vec<String> {
    let re = Regex::new(r"[^\w, ]+").unwrap();
    let gram = gram_size.unwrap_or(2);
    let simplified = format!("-{}-", &re.replace(&value.to_lowercase(), ""));
    let mut results = Vec::new();

    for i in 0..=simplified.chars().count() - gram {
        let mut sub_string = String::new();
        for l in i..(i + gram) {
            sub_string.push(simplified.chars().nth(l).unwrap())
        }
        results.push(sub_string);
    }
    return results;
}

pub fn gram_counter(value: &str, gram_size: Option<usize>) -> HashMap<String, usize> {
    // return an object where key=gram, value=number of occurrences
    let mut result: HashMap<String, usize> = HashMap::new();
    let grams = iterate_grams(&value, gram_size);

    for i in grams {
        result
            .entry(i)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
    return result;
}

#[derive(Debug)]
struct FuzzySet {
    exact_set: HashMap<String, String>,
    items: HashMap<usize, Vec<(f32, String)>>,
    match_dict: HashMap<String, Vec<(usize, usize)>>,
}

const MAX_RESULTS_SIZE: usize = 50;

impl FuzzySet {
    fn new(arr: Vec<&str>) -> Self {
        let mut fz = Self {
            exact_set: HashMap::new(),
            items: HashMap::new(),
            match_dict: HashMap::new(),
        };

        for word in arr {
            fz.add(&word)
        }

        fz
    }

    fn add<'a>(&mut self, value: &'a str) {
        if let Some(_) = self.exact_set.get(&value.to_lowercase()) {
            return;
        }

        for i in GRAM_SIZE_LOWER..=GRAM_SIZE_UPPER {
            self._add(value, i);
        }
        return;
    }

    fn _add(&mut self, value: &str, gram_size: usize) {
        let normalized_value = value.to_lowercase();
        let mut add_items = self
            .items
            .get(&gram_size)
            .unwrap_or(&Vec::<(f32, String)>::with_capacity(10))
            .to_owned();
        let index = add_items.len();

        let gram_counts = gram_counter(&normalized_value, Some(gram_size));
        let mut sum_of_square_gram_counts = 0;

        for (gram, gram_count) in gram_counts {
            sum_of_square_gram_counts += gram_count.pow(2);
            self.match_dict
                .entry(gram)
                .and_modify(|m| m.push((index, gram_count)))
                .or_insert(vec![(index, gram_count)]);
        }

        let vector_normal = (sum_of_square_gram_counts as f32).powf(0.5);
        add_items.insert(index, (vector_normal, normalized_value.clone()));
        self.items.insert(gram_size, add_items);
        self.exact_set.insert(normalized_value, value.to_string());
    }

    fn get(&self, value: &str) -> Option<Vec<(f32, String)>> {
        let normalized_value = value.to_lowercase();
        if let Some(result) = self.exact_set.get(&normalized_value) {
            return Some(vec![(1.0, result.to_string())]);
        }

        for gs in (GRAM_SIZE_LOWER..=GRAM_SIZE_UPPER).rev() {
            // this is a bug as _get() never returns a None but acheives parity with the existing code
            if let Some(results) = self._get(value, gs) {
                return Some(results);
            }
        }

        return None;
    }

    fn _get(&self, value: &str, gram_size: usize) -> Option<Vec<(f32, String)>> {
        let normalized_value = value.to_lowercase();
        let mut matches: HashMap<usize, usize> = HashMap::new();
        let gram_counts = gram_counter(&normalized_value, Some(gram_size));
        let items = self.items.get(&gram_size).unwrap();
        let mut sum_of_square_gram_counts: f32 = 0.0;

        for (gram, gram_count) in gram_counts {
            sum_of_square_gram_counts += gram_count.pow(2) as f32;
            if let Some(result) = self.match_dict.get(&gram) {
                for (index, other_gram_count) in result {
                    let product = gram_count * other_gram_count;
                    matches
                        .entry(*index)
                        .and_modify(|m| *m += product)
                        .or_insert(product);
                }
            }
        }

        let vector_normal = sum_of_square_gram_counts.powf(0.5);
        let mut results: Vec<(f32, String)> = Vec::new();

        for (match_index, match_score) in matches {
            if let Some(found_item) = items.get(match_index) {
                results.push((
                    (match_score as f32) / (vector_normal * found_item.0),
                    found_item.1.clone(),
                ))
            }
        }

        results.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(Equal));

        let mut new_results: Vec<(f32, String)> = Vec::new();

        for (index, result) in results.iter().enumerate() {
            if index < MAX_RESULTS_SIZE {
                new_results.push((
                    distance(Some(&result.1), Some(&normalized_value)).expect("err"),
                    result.1.clone(),
                ))
            }
        }

        results = new_results;
        results.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(Equal));

        new_results = Vec::new();

        for (index, value) in results.iter().enumerate() {
            if value.0 == results[0].0 {
                new_results.push((results[index].0, self.exact_set[&results[index].1].clone()))
            }
        }

        return Some(new_results);
    }
}

#[cfg(test)]
mod tests {

    use std::fs::{self};

    use super::*;

    #[test]
    fn test_levenshtein() {
        assert_eq!(levenshtein("cats", "dogs"), 3);
        assert_eq!(levenshtein("barbeque", "arse"), 5);
        assert_eq!(levenshtein("ping pong", "walkie talkie"), 12);
        assert_eq!(levenshtein("everest", "k2"), 7);
        assert_eq!(levenshtein("California", "New York"), 8);
        assert_eq!(levenshtein("california", "new york"), 8);
    }

    macro_rules! assert_delta {
        ($x:expr, $y:expr, $d:expr) => {
            if !($x - $y < $d || $y - $x < $d) {
                panic!();
            }
        };
    }

    #[test]
    fn test_distance() {
        assert_eq!(distance(Some("cats"), Some("dogs")), Ok(0.25));
        assert_eq!(distance(Some("barbeque"), Some("arse")), Ok(0.375));
        assert_delta!(
            distance(Some("ping pong"), Some("walkie talkie")).unwrap(),
            0.07692307692307687,
            0.00001
        );
        assert_eq!(distance(Some("everest"), Some("k2")), Ok(0.0));
        assert_delta!(
            distance(Some("California"), Some("New York")).unwrap(),
            0.19999999999999996,
            0.00001
        );
        assert_delta!(
            distance(Some("california"), Some("new york")).unwrap(),
            0.19999999999999996,
            0.00001
        );
    }

    #[test]
    fn test_iterate_grams() {
        assert_eq!(
            iterate_grams("cats", Some(1)),
            ["-", "c", "a", "t", "s", "-"]
        );
        assert_eq!(
            iterate_grams("barbeque", Some(2)),
            ["-b", "ba", "ar", "rb", "be", "eq", "qu", "ue", "e-"]
        );
        assert_eq!(
            iterate_grams("ping pong", Some(3)),
            ["-pi", "pin", "ing", "ng ", "g p", " po", "pon", "ong", "ng-"]
        );
        assert_eq!(
            iterate_grams("everest", Some(4)),
            ["-eve", "ever", "vere", "eres", "rest", "est-"]
        );
        assert_eq!(
            iterate_grams("California", Some(5)),
            ["-cali", "calif", "alifo", "lifor", "iforn", "forni", "ornia", "rnia-"]
        );
        assert_eq!(
            iterate_grams("california", Some(6)),
            ["-calif", "califo", "alifor", "liforn", "iforni", "fornia", "ornia-"]
        );
        assert_eq!(
            iterate_grams("a!b@c£d$$ **e(f)g", Some(3)),
            [
                "-ab", "ab@", "b@c", "@c£", "c£d", "£d$", "d$$", "$$ ", "$ *", " **", "**e", "*e(",
                "e(f", "(f)", "f)g", ")g-"
            ]
        )
    }

    macro_rules! assert_hashmap_eq {
        ($left:expr, $right:expr $(,)?) => {
            match (&$left, &$right) {
                (left_val, right_val) => {
                    for (key, val) in left_val.iter() {
                        match right_val.get(key) {
                            None => assert_eq!(1, 2),
                            Some(v) => assert_eq!(val, v),
                        }
                    }
                }
            }
        };
    }

    #[test]
    fn test_gram_counter() {
        assert_hashmap_eq!(
            gram_counter("cats", Some(1)),
            HashMap::from([
                (String::from("-"), 2),
                (String::from("c"), 1),
                (String::from("a"), 1),
                (String::from("t"), 1),
                (String::from("s"), 1),
            ])
        );
        assert_eq!(
            gram_counter("barbeque", Some(2)),
            HashMap::from([
                (String::from("-b"), 1),
                (String::from("ba"), 1),
                (String::from("ar"), 1),
                (String::from("rb"), 1),
                (String::from("be"), 1),
                (String::from("eq"), 1),
                (String::from("qu"), 1),
                (String::from("ue"), 1),
                (String::from("e-"), 1)
            ])
        );
        assert_eq!(
            gram_counter("ping pong", Some(3)),
            HashMap::from([
                (String::from("-pi"), 1),
                (String::from("pin"), 1),
                (String::from("ing"), 1),
                (String::from("ng "), 1),
                (String::from("g p"), 1),
                (String::from(" po"), 1),
                (String::from("pon"), 1),
                (String::from("ong"), 1),
                (String::from("ng-"), 1),
            ])
        );
        assert_eq!(
            gram_counter("everest", Some(4)),
            HashMap::from([
                (String::from("-eve"), 1),
                (String::from("ever"), 1),
                (String::from("vere"), 1),
                (String::from("eres"), 1),
                (String::from("rest"), 1),
                (String::from("est-"), 1)
            ])
        );
        assert_eq!(
            gram_counter("California", Some(5)),
            HashMap::from([
                (String::from("-cali"), 1),
                (String::from("calif"), 1),
                (String::from("alifo"), 1),
                (String::from("lifor"), 1),
                (String::from("iforn"), 1),
                (String::from("forni"), 1),
                (String::from("ornia"), 1),
                (String::from("rnia-"), 1)
            ])
        );
        assert_eq!(
            gram_counter("a!b@c£d$$ **e(f)g", Some(3)),
            HashMap::from([
                (String::from("-ab"), 1),
                (String::from("ab@"), 1),
                (String::from("b@c"), 1),
                (String::from("@c£"), 1),
                (String::from("c£d"), 1),
                (String::from("£d$"), 1),
                (String::from("d$$"), 1),
                (String::from("$$ "), 1),
                (String::from("$ *"), 1),
                (String::from(" **"), 1),
                (String::from("**e"), 1),
                (String::from("*e("), 1),
                (String::from("e(f"), 1),
                (String::from("(f)"), 1),
                (String::from("f)g"), 1),
                (String::from(")g-"), 1)
            ])
        )
    }

    #[test]
    fn test_fuzzyset() {
        let file = fs::read_to_string("src/fuzzymatch_test.json").expect("couldn't read file");

        let json: serde_json::Value = serde_json::from_str(&file).expect("JSON not well formatted");

        // print!("{:?}", json.get("exact_set"));

        let fz = FuzzySet::new(vec!["Parrot", "Dog", "Capy Bara"]);
        // println!("{:?}", fz.exact_set);
        for (key, value) in fz.exact_set {
            assert_eq!(&value, json.get("exact_set").unwrap().get(key).unwrap());
        }
        for (key, value) in fz.items {
            for (index, item) in value.iter().enumerate() {
                assert_eq!(
                    item.1,
                    json.get("items").unwrap().get(key.to_string()).unwrap()[index][1]
                );
            }
        }
        for (key, value) in fz.match_dict {
            for (index, item) in value.iter().enumerate() {
                assert_eq!(
                    item.1,
                    json.get("match_dict").unwrap().get(key.clone()).unwrap()[index][1],
                );
            }
        }
    }

    #[test]
    fn test_fuzzymatch() {
        let needle = "Cats";
        let mut haystack = vec!["Parrot", "Dog", "Capy Bara"];
        assert_eq!(fuzzymatch(needle, haystack), None);

        haystack = vec!["Parrot", "Dog", "Capy Bara", "Cat"];
        assert_eq!(fuzzymatch(needle, haystack), Some("Cat".to_string()));
    }
}
