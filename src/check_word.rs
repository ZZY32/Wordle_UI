use std::collections::HashMap;

use serde_derive::{Serialize, Deserialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum MyColor {
    Green,
    Yellow,
    Red,
    Unknown
}
#[derive(Clone, Serialize, Deserialize)]
pub struct GameState {
    pub total_rounds : i64,
    pub correct_rounds : i64,
    pub games: Vec<Game>,
    pub used_words : HashMap<String, i32>,
    pub random_index : Vec<usize>,
    pub is_normal : bool,
    pub this_time_total_rounds : i64
}
#[derive(Clone, Serialize, Deserialize)]
pub struct Game {
    pub answer : String,
    pub guesses : Vec<(String, Vec<MyColor>)>,
    pub key_state : Vec<i32>,
}

pub fn check_word (answer_word : &String, guess_word : &String) -> (Vec<MyColor>, Vec<i32>){
    let mut alp_state = vec![-1; 26];
    let mut answer_chars: Vec<char> = Vec::new();
    for i in answer_word.chars() {
        answer_chars.push(i);
    }
    let mut guess_chars: Vec<char> = Vec::new();
    for i in guess_word.chars() {
        guess_chars.push(i);
    }

    //let answer_chars_sub  = answer_chars.clone();
    let guess_chars_sub  = guess_chars.clone();
    let mut output : Vec<MyColor> = Vec::from([MyColor::Unknown; 5]);
    for i in 0..5 {
        if &answer_chars[i] == &guess_chars[i] {
            output[i] = MyColor::Green;
            answer_chars[i] = '0';
            guess_chars[i] = '0';
        }
    }
    for i in 0..5 {
        if &guess_chars[i] != &'0' {
            for j in 0..5 {
                if &guess_chars[i] == &answer_chars[j] {
                    output[i] = MyColor::Yellow;
                    guess_chars[i] = '0';
                    answer_chars[j] = '0';
                    break;
                }
            }
        }
    }
    for i in 0..5 {
        match &output[i] {
            &MyColor::Unknown => output[i] = MyColor::Red,
            _ => {}
        }
    }

    for i in 0..5 {
        match &output[i] {
            &MyColor::Green => {
                alp_state[(guess_chars_sub[i] as usize) - 97] = 2;
            }
            &MyColor::Yellow => {
                alp_state[(guess_chars_sub[i] as usize) - 97] = if alp_state[(guess_chars_sub[i] as usize) - 97] < 1 {
                    1
                } else {
                    alp_state[(guess_chars_sub[i] as usize) - 97]
                }
            }
            &MyColor::Red => {
                alp_state[(guess_chars_sub[i] as usize) - 97] = if alp_state[(guess_chars_sub[i] as usize) - 97] < 0 {
                    0
                } else {
                    alp_state[(guess_chars_sub[i] as usize) - 97]
                }
            }
            _ => unimplemented!()
        }
    }
    (output, alp_state)
}