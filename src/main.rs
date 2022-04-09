use yew::prelude::*;
use yew::{Component, Context, html, Html};
use rand::Rng;
use core::ops::Range;
use urlencoding;

pub enum UIMsg {
    Clicked
}

struct MyComponent {
}

impl Component for MyComponent {
    type Message = UIMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            UIMsg::Clicked => {
                return true // Re-render
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let kanji_val_range = 1..101;
        let hiragana_val_range = 1..101;

        let zen = format!("{0}条禅", &rand_val_kanji(kanji_val_range.clone()));
        let nanako = format!("{0}条{1}こ", &rand_val_kanji(kanji_val_range.clone()), &rand_val_hiragana(hiragana_val_range.clone()));

        html!{
            <div>
                <h1>{ zen }</h1>
                <h1>{ nanako }</h1>
                <br/>
                <button onclick={_ctx.link().callback(|_| UIMsg::Clicked)}>{ "リロール" }</button>
            </div>
        }
    }

}

fn num_to_str(num: i32, str_nums: &[&str; 10], str_jyu: &str, str_hyaku: &str) -> String {
    if num == 100 {return String::from(str_hyaku);}

    return format!("{0}{1}{2}",
                   if num >= 20 {str_nums[(num/10) as usize]} else {""},
                   if num >= 10 {str_jyu} else {""},
                   str_nums[(num%10) as usize]);
}

fn num_to_hiragana(num: i32) -> String {
    let hiragana_hyaku = "ひゃく";
    let hiragana_jyu = "じゅう";
    let hiragana_nums: [&str; 10] = [
        "",
        "いち",
        "に",
        "さん",
        "よん",
        "ご",
        "ろく",
        "なな",
        "はち",
        "きゅう",
    ];

    return num_to_str(num, &hiragana_nums, hiragana_jyu, hiragana_hyaku);
}

fn num_to_kanji(num: i32) -> String {
    let hiragana_hyaku = "百";
    let hiragana_jyu = "十";
    let hiragana_nums: [&str; 10] = [
        "",
        "一",
        "二",
        "三",
        "四",
        "五",
        "六",
        "七",
        "八",
        "九",
    ];

    return num_to_str(num, &hiragana_nums, hiragana_jyu, hiragana_hyaku);
}

fn rand_val_hiragana(val_range: Range<i32>) -> String {
    let secret_number = rand::thread_rng().gen_range(val_range);
    let hiragana_number = num_to_hiragana(secret_number);

    return format!("{hiragana_number}");
}

fn rand_val_kanji(val_range: Range<i32>) -> String {
    let secret_number = rand::thread_rng().gen_range(val_range);
    let hiragana_number = num_to_kanji(secret_number);

    return format!("{hiragana_number}");
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <MyComponent/>
    }
}

fn main() {
    yew::start_app::<App>();
}