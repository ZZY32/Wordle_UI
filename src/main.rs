use std::cmp::min;
//use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;



//use cursive::backends::curses::n::ncurses::OK;
use cursive::menu;
use cursive::theme::BaseColor;
use cursive::theme::Color;
//use cursive::theme::Effect;
//use cursive::theme::Style;
use cursive::utils::markup::StyledString;
//use cursive::vec;
use cursive::view::{Resizable, Nameable};
use cursive::{Cursive, CursiveExt, With};
use cursive::views::{TextView, Dialog, EditView, ListView, LinearLayout, Button, DummyView};


mod builtin_words;
extern crate rand;
use builtin_words::FINAL;
use builtin_words::ACCEPTABLE;

use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};


mod check_word;
use check_word::{MyColor, Game, GameState, check_word};


// 采用随机模式

fn main() -> Result<(), serde_json::Error> {
    // 新建UI
    let mut siv = Cursive::new();
    //let answer_word = "apple";
    siv.load_toml(include_str!("../theme.toml")).unwrap();

    // 获取随机下标序列
    let mut index_arr : Vec<usize> = Vec::new();
    for i in 0..FINAL.len() as usize {
        index_arr.push(i);
    }
    let mut rand_engine = StdRng::seed_from_u64(rand::thread_rng().gen());

    index_arr.shuffle(&mut rand_engine);
    

    // 加载游戏状态
    // 新建游戏状态
    let mut game_state : GameState ;
    // 从json文件中读取状态
    let mut f = File::open("src/state.json").unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();
    game_state = serde_json::from_str(&buf)?;

    // 重置一些状态
    game_state.random_index = Vec::new();
    for i in index_arr {
        game_state.random_index.push(i);
    }
    game_state.is_normal = true;
    game_state.this_time_total_rounds = 0;
    // 传入游戏状态
    siv.set_user_data(game_state);

    // 设置菜单
    siv.menubar()
    .add_subtree(
        "mode", 
        menu::Tree::new()
        .leaf("Normal", move |s|{
            s.with_user_data(|user_data: &mut GameState| {
                user_data.is_normal = true;
            });
        })
        .leaf("Difficult", move |s|{
            s.with_user_data(|user_data: &mut GameState| {
                user_data.is_normal = false;
            });
        })
    )
    .add_subtree(
        "statics", 
        menu::Tree::new()
        .leaf("show", move |s|{
            show_statics(s);
        })
        .leaf("clear", move |_s|{
            let temp = 
            "{
                \"total_rounds\" : 0,
                \"correct_rounds\" : 0,
                \"games\" : [],
                \"key_state\" : [],
                \"used_words\" : {},
                \"random_index\" : [],
                \"is_normal\" : true,
                \"this_time_total_rounds\" : 0
            }"
            ;
            let mut f = File::create("src/state.json").unwrap();
            match f.write_all(temp.as_bytes()) {
                Err(_) => {
                    unimplemented!("State file error")
                }
                _ => {}
            }
        })
    );
    siv.set_autohide_menu(false);
    siv.select_menubar();
    /*
    siv.add_layer(
        Dialog::around(EditView::new().fixed_width(10))
        .title("Please Enter")
        .button("Ok", |s| s.quit())
        .button("cancel", |s| s.quit())
    );
    siv.add_global_callback('q', |s| s.quit());
    */
    //let menu = Menubar::new();
    //开始游戏循环
        //if index == FINAL.len() {
        //    break;
        //}
        // 开始一局游戏
        //let answer_word : String = FINAL[index_arr[index]].to_string();
        // 开始猜词

    // 开始游戏
    show_ui_game(&mut siv, true);
    siv.run();
    Ok(())
}
        
    


// 展示数据
fn show_statics(s : &mut Cursive) {
    // 总局数
    let t_r = s.user_data::<GameState>().unwrap().total_rounds;
    // 成功局数
    let c_r = s.user_data::<GameState>().unwrap().correct_rounds;
    // 用过的单词
    let mut used_words_tup : Vec<(String, i32)> = Vec::new();
    let used_words_sub = s.user_data::<GameState>().unwrap().used_words.clone();
    for (word, times) in used_words_sub {
        used_words_tup.push((word, times));
    }
    for i in 0..used_words_tup.len() {
        for j in 0..(used_words_tup.len()-i-1) {
            if used_words_tup[j].1 < used_words_tup[j+1].1 {
                let temp = used_words_tup[j].clone();
                used_words_tup[j] = used_words_tup[j+1].clone();
                used_words_tup[j+1] = temp;
            } else if used_words_tup[j].1 == used_words_tup[j+1].1 {
                if used_words_tup[j].0 > used_words_tup[j+1].0 {
                    let temp = used_words_tup[j].clone();
                    used_words_tup[j] = used_words_tup[j+1].clone();
                    used_words_tup[j+1] = temp;
                }
            }
        }
    }
    let info = LinearLayout::vertical()
        .child(TextView::new(format!("Total: {}", t_r)))
        .child(TextView::new(format!("Correct: {}", c_r)))
        .child(Dialog::around(ListView::new()
            .with(
                |list| {
                    for i in 0..min(5, used_words_tup.len()){
                        list.add_child(&used_words_tup[i].0[..], TextView::new(format!("{}", used_words_tup[i].1)))
                    }
                }
            ))
            .title("WORDS AND USED TIMES")
        )
        ;
    s.add_layer(Dialog::around(
        info
        )
        .title("STATICS")
        .button("OK", |s| {s.pop_layer();})
    )
    /*s.with_user_data(|user_data: &mut GameState| {
        let info = LinearLayout::vertical()
        .child(TextView::new(format!("Total: {}", user_data.total_rounds)))
        .child(TextView::new(format!("Correct: {}", user_data.correct_rounds)))
        ;
    });*/
}

// 处理游戏逻辑
fn run_game (siv : &mut Cursive) {
    // 处理游戏逻辑
        // 获取猜测的词
        let guess_word = siv.call_on_name("input", |view : &mut EditView|{
            view.get_content()
        }).unwrap().to_string().to_lowercase();
        //let guess_word = String::from("aback");
        /*siv.add_layer(Dialog::around(
            TextView::new(format!("{guess_word}"))
            )
            .button("ok", |siv| {siv.pop_layer();})
        );
        */
        // 判断词是否合法
        let mut is_qualified = true;
        if !ACCEPTABLE.contains(&&guess_word.trim()[..]) {
            is_qualified = false;
        }
        if is_qualified {
            // 困难模式
            if !siv.user_data::<GameState>().unwrap().is_normal {
                let mut guess_chars : Vec<char> = Vec::new();
                for i in guess_word.trim().chars() {
                    guess_chars.push(i);
                }
                let last_index = siv.user_data::<GameState>().unwrap().games.clone().len()-1;
                if siv.user_data::<GameState>().unwrap().games[last_index].guesses.len() == 0 {
                    is_qualified = true;
                } else {
                    let mut last_chars : Vec<char> = Vec::new();
                    let mut last_color : Vec<MyColor> = Vec::new();
                    let index = siv.user_data::<GameState>().unwrap().games[last_index].guesses.len()-1;
                    for color in &siv.user_data::<GameState>().unwrap().games[last_index].guesses[index].1 {
                        last_color.push(color.clone());
                    }
                    for c in siv.user_data::<GameState>().unwrap().games[last_index].guesses[index].0.chars() {
                        last_chars.push(c);
                    }
                    for i in 0..5 {
                        match last_color[i] {
                            MyColor::Green => {
                                if guess_chars[i] != last_chars[i] {
                                    is_qualified = false;
                                    break;
                                } else {
                                    guess_chars[i] = '0';
                                    continue;
                                }
                            }
                            _ => {}
                        }
                    }
                    for i in 0..5 {
                        match last_color[i] {
                            MyColor::Yellow => {
                                let mut is_yellow_qualified = false;
                                for j in 0..5 {
                                    if last_chars[i] == guess_chars[j] {
                                        guess_chars[j] = '0';
                                        is_yellow_qualified = true;
                                        break;
                                    }
                                }
                                if !is_yellow_qualified {
                                    is_qualified = false;
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }
                
                }
            }
        }
        if is_qualified {
            // 猜词合法
            let outcome = check_word(&siv.with_user_data(|user_data: &mut GameState| {
                user_data.games.last().as_ref().unwrap().answer.clone()
            }).unwrap(), &guess_word);

            siv.with_user_data(|user_data: &mut GameState| {
                let count = user_data.used_words.entry(guess_word.clone()).or_insert(0);
                *count += 1;

                let last_index = user_data.games.clone().len()-1;
                for i in 0..26 as usize {
                    if user_data.games[last_index].key_state[i] < outcome.1[i] {
                        user_data.games[last_index].key_state[i] = outcome.1[i];
                    }
                }

                user_data.games[last_index].guesses.push((guess_word.clone(),outcome.0));
            });

            //siv.with_user_data(|user_data: &mut GameState| {
            //    
            //});

            //siv.with_user_data(|user_data: &mut GameState| {
            //   let last_index = user_data.games.clone().len()-1;
            //        
            //});
            //let counts = siv.with_user_data(|user_data: &mut GameState| {
            //    user_data.used_words.entry(guess_word.clone()).or_insert(0)
            //});
            let answer_word = siv.with_user_data(|user_data: &mut GameState|{
                let last_index = user_data.games.clone().len()-1;
                user_data.games[last_index].answer.clone()
            }).unwrap();
            if answer_word == guess_word {
                // 猜测成功
                siv.with_user_data(|user_data: &mut GameState| {
                    user_data.correct_rounds += 1;
                    user_data.total_rounds += 1;
                    user_data.this_time_total_rounds += 1;
                });

                // 将数据写入文件
                let temp = serde_json::to_string_pretty(&siv.user_data::<GameState>().unwrap());
                let mut f = File::create("src/state.json").unwrap();
                match f.write_all(temp.unwrap().as_bytes()) {
                    Err(_) => unimplemented!(),
                    _ => {}
                }
                
                // 询问是否开始新一局
                siv.add_layer(Dialog::around(TextView::new("CORRECT"))
                .button("CANCEL", Cursive::quit)
                .button("NEXT", |siv| {
                        siv.pop_layer();
                        siv.pop_layer();
                        show_ui_game(siv, true);
                    })
                )
            } else {
                // 猜词失败
                let guess_times = siv.with_user_data(|user_data: &mut GameState|{
                    let last_index = user_data.games.clone().len()-1;
                    user_data.games[last_index].guesses.len()
                });
                if guess_times.unwrap() == 6 {
                    // 失败六次 游戏结束
                    siv.with_user_data(|user_data: &mut GameState| {
                        user_data.total_rounds += 1;
                        user_data.this_time_total_rounds += 1;
                    });

                    // 将数据写入文件
                    let temp = serde_json::to_string_pretty(&siv.user_data::<GameState>().unwrap());
                    let mut f = File::create("src/state.json").unwrap();
                    match f.write_all(temp.unwrap().as_bytes()) {
                        Err(_) => unimplemented!(),
                        _ => {}
                    }

                    // 询问是否开始新一局
                    siv.add_layer(Dialog::around(TextView::new(format!("ERROR! THE CORRECT ANSWER {}", answer_word.to_uppercase())))
                    .button("CANCEL", Cursive::quit)
                    .button("NEXT", |siv|{
                            siv.pop_layer();
                            siv.pop_layer();
                            show_ui_game(siv, true);
                        })
                    );
                    //show_ui_game(siv, true);
                } else {
                    // 失败不足六次 询问是否继续继续猜词
                    siv.add_layer(Dialog::around(TextView::new("ERROR! TRY AGAIN!"))
                    .button("CANCEL", Cursive::quit)
                    .button("NEXT", |siv|{
                            siv.pop_layer();
                            siv.pop_layer();
                            show_ui_game(siv, false)
                        })
                    );
                    //show_ui_game(siv, false);
                }
            }
        } else {
            // 无效猜词
            siv.add_layer(Dialog::around(
                TextView::new(guess_word)
            )
                .title("INVALID INPUT")
                .button("OK", |siv| {
                    siv.pop_layer();
                })
            )     
        }
}

// 根据是否为新一局游戏 加载界面
fn show_ui_game (siv : &mut Cursive, is_new : bool) {
    
    siv.with_user_data(|user_data: &mut GameState| {
        if user_data.total_rounds as usize == FINAL.len() {
            Cursive::quit;
        }
    });
    // 新一局游戏 新建一局游戏状态并加入user_data 否则直接加载user_data中最后一局游戏的状态
    if is_new {
        let total_game = 
        siv.with_user_data(|user_data: &mut GameState| {
            user_data.this_time_total_rounds
        });
        let index = 
        siv.with_user_data(|user_data: &mut GameState| {
            user_data.random_index[total_game.unwrap() as usize]
        });
        let new_game = Game { 
            answer : FINAL[index.unwrap()].to_string(), 
            guesses : Vec::new(),  
            key_state : Vec::from([-1; 26])
        };
        siv.with_user_data(|user_data: &mut GameState| {
            user_data.games.push(new_game);
        });
    }



    // 本局用过的词的列表
    let list = ListView::new()
    .with(|list|{
        siv.with_user_data(|user_data: &mut GameState|{
            let last_index = user_data.games.clone().len()-1;
            for i in &user_data.games[last_index].guesses {
                let mut styled_word = StyledString::new();
                let mut word_char : Vec<char> = Vec::new();
                for c in i.0.chars() {
                    word_char.push(c);
                }
                for j in 0..5 {
                    styled_word.append(StyledString::styled(word_char[j].to_string(), match i.1[j] {
                        MyColor::Green => Color::Light(BaseColor::Green),
                        MyColor::Yellow => Color::Dark(BaseColor::Yellow),
                        MyColor::Red => Color::Light(BaseColor::Red),
                        _ => Color::Dark(BaseColor::Black)
                    }))
                }

                list.add_child("·", TextView::new(styled_word));
            }
        });
    })
    .with_name("used_word")
    .fixed_width(20);
    let guesses_list = Dialog::new().content(list)
    .title("THE WORD YOU GUESS");
    



    // 输入框
    let input = Dialog::new()
    .content(
        EditView::new()
        .with_name("input")
        .fixed_width(45),
    )
    .title("Please Enter the word you guess!")
    .button("Ok", |siv| {
        // 处理游戏逻辑
        run_game(siv);
    });
    //.button("cancel", |s| s.quit());




    // 键盘区
    // 加载键盘区颜色
    let temp = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut key_chars : Vec<char> = Vec::new();
    for c in temp.chars() {
        key_chars.push(c);
    }
    let mut styled_chars : Vec<StyledString> = Vec::new();

    siv.with_user_data(|user_data: &mut GameState| {
        for i in 0..26 as usize {
            let last_index = user_data.games.clone().len()-1;
            styled_chars.push(StyledString::styled(key_chars[i].to_string(), 
            match user_data.games[last_index].key_state[i] {
                -1 => Color::Dark(BaseColor::Black),
                0 => Color::Dark(BaseColor::Red),
                1 => Color::Dark(BaseColor::Yellow),
                2 => Color::Dark(BaseColor::Green),
                _ => unimplemented!()
            }
        ))
        }
    });
    let key_width = 9;
    let key_board = Dialog::around(LinearLayout::vertical()
    .child(LinearLayout::horizontal().child(TextView::new(styled_chars[0].clone()).fixed_width(key_width)).child(TextView::new(styled_chars[1].clone()).fixed_width(key_width)).child(TextView::new(styled_chars[2].clone()).fixed_width(key_width)).child(TextView::new(styled_chars[3].clone()).fixed_width(key_width)).child(TextView::new(styled_chars[4].clone()).fixed_width(key_width)))
    .child(LinearLayout::horizontal().child(TextView::new(styled_chars[5].clone()).fixed_width(key_width)).child(TextView::new(styled_chars[6].clone()).fixed_width(key_width)).child(TextView::new(styled_chars[7].clone()).fixed_width(key_width)).child(TextView::new(styled_chars[8].clone()).fixed_width(key_width)).child(TextView::new(styled_chars[9].clone()).fixed_width(key_width)))
    .child(LinearLayout::horizontal().child(TextView::new(styled_chars[10].clone()).fixed_width(key_width)).child(TextView::new(styled_chars[11].clone()).fixed_width(key_width)).child(TextView::new(styled_chars[12].clone()).fixed_width(key_width)).child(TextView::new(styled_chars[13].clone()).fixed_width(key_width)).child(TextView::new(styled_chars[14].clone()).fixed_width(key_width)))
    .child(LinearLayout::horizontal().child(TextView::new(styled_chars[15].clone()).fixed_width(key_width)).child(TextView::new(styled_chars[16].clone()).fixed_width(key_width)).child(TextView::new(styled_chars[17].clone()).fixed_width(key_width)).child(TextView::new(styled_chars[18].clone()).fixed_width(key_width)).child(TextView::new(styled_chars[19].clone()).fixed_width(key_width)))
    .child(LinearLayout::horizontal().child(TextView::new(styled_chars[20].clone()).fixed_width(key_width)).child(TextView::new(styled_chars[21].clone()).fixed_width(key_width)).child(TextView::new(styled_chars[22].clone()).fixed_width(key_width)).child(TextView::new(styled_chars[23].clone()).fixed_width(key_width)).child(TextView::new(styled_chars[24].clone()).fixed_width(key_width)))
    .child(LinearLayout::horizontal().child(TextView::new(styled_chars[25].clone()).fixed_width(key_width)))
    )
    .title("KEYBOARD");



    // 底部按钮
    let buttons = LinearLayout::horizontal()
    .child(DummyView.fixed_width(40))
    //.child(Button::new("Statics", show_statics))
    //.child(DummyView.fixed_width(2))
    .child(Button::new("Quit", Cursive::quit));


    
    // 界面右侧区域 包括输入框 键盘区 底部按钮
    let right = LinearLayout::vertical()
    .child(input)
    .child(key_board)
    .child(buttons);
    


    siv.add_layer(Dialog::around(LinearLayout::horizontal()
    .child(guesses_list)
    .child(right))
    .title("WORDLE")
    );
}