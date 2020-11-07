use std::env;
use std::str;

#[rustfmt::skip]
fn to_base(n: i64,
           base: i64,
           alph_a: Vec<&str>,
           alph_b: Vec<&str>) -> Vec<Vec<String>> {
  let mut res_a: Vec<String> = Vec::new();
  let mut res_b: Vec<String> = Vec::new();
  let mut num: i64 = n;
  while num != 0 {
      let mut index: i64 = num % base;
      num /= base;
      if index < 0 {
          index += base.abs();
          num += 1;
      }
      if base > 36 {
        res_a.push(alph_a[index as usize].to_string());
      } else if base < 0 {
        res_a.push(index.to_string());
      } else {
        res_a.push(alph_b[index as usize].to_string())
      }
      res_b.push(index.to_string());
  }
  res_a.reverse();
  res_b.reverse();
  return vec![res_a, res_b];
}

#[rustfmt::skip]
#[test]
#[ignore = "not implemented"]
fn roman() {
  // ...
}

#[rustfmt::skip]
fn main() {
  let color1: &str = "\x1B[34m";
  let color2: &str = "\x1B[32m";
  let color3: &str = "\x1B[33m";
  let creset: &str = "\x1B[0m";
  let bases: Vec<(&str, i64, &str)> = 
      vec![("negadecimal",  -10, "nd"),
           ("binary",         2,  "b"),
           ("ternary",        3, "te"),
           ("quaternary",     4, "qa"),
           ("quinary",        5, "qi"),
           ("senary",         6, "se"),
           ("octal",          8,  "o"),
           ("duodecimal",    12,  "d"),
           ("hexadecimal",   16,  "x"),
           ("vigesimal",     20,  "v"),
           ("duotrigesimal", 32, "td"),
           ("sexagesimal",   60,  "s")];
  let args:       Vec<String> = env::args().collect(); // get argv[...]
  let decimal:        &String = &args[1];              // number to translate
  let mut decimal_int:    i64 = decimal.parse::<i64>().unwrap();
  let decimal_int_r:      i64 = decimal_int;
  let alphabet_a:   Vec<&str> = 
      vec!["0", "1", "2", "3", "4", // base > 36 alphabet
           "5", "6", "7", "8", "9",
           "ā", "b", "c", "d", "ē",
           "f", "g", "h", "ī", "j",
           "k", "l", "m", "n", "ō",
           "p", "q", "r", "s", "t",
           "ū", "v", "w", "x", "ȳ",
           "z", "α", "β", "γ", "δ",
           "ε", "ζ", "η", "θ", "ι",
           "κ", "λ", "μ", "ν", "ξ",
           "ο", "π", "ρ", "σ", "τ",
           "υ", "φ", "χ", "ψ", "ω"];
  let alphabet_b:   Vec<&str> = 
      vec!["0", "1", "2", "3", "4", // universal alphabet
           "5", "6", "7", "8", "9",
           "a", "b", "c", "d", "e",
           "f", "g", "h", "i", "j",
           "k", "l", "m", "n", "o",
           "p", "q", "r", "s", "t",
           "u", "v", "w", "x", "y",
           "z"];
  for (name, base, prefix_str) in bases {
    let numbers: Vec<Vec<String>>;
    let number_a_str:      String;
    let number_b_str:      String;
    let negate:            String;
    let prefix:            String = ["0", prefix_str].join("");
    if decimal_int_r < 0 && base > 0 {
      negate = "-".to_string();
    } else {
      negate = "".to_string();
    }
    if base < 0 {
      decimal_int = decimal_int_r;
    } else {
      decimal_int = decimal_int_r.abs();
    }
    loop {
      numbers = to_base(decimal_int,
                        base, 
                        alphabet_a.clone(),
                        alphabet_b.clone());
      number_a_str = numbers[0].join("");
      number_b_str = numbers[1].join(";");
      break;
    }
    println!("{}// {}[{}]{}\t{}{}{}",
             color2,
             color3,
             prefix_str.to_uppercase(),
             creset,
             color1,
             name,
             creset);
    println!("{}{}{}\t{}{}",
             negate,
             prefix,
             number_a_str,
             negate,
             number_a_str);
    if base > 10 {
      println!("{}{}{}\t{}{}",
               negate,
               prefix,
               number_a_str.to_uppercase(),
               negate,
               number_a_str.to_uppercase());
    }
    if base > 36 {
      println!("\t{}{}\t{}-- Using only decimal numbers{}",
               negate,
               number_b_str,
               color2,
               creset);
    }
    println!();
  }
}
