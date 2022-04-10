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

        let zen_base_app = 160;
        let zen_number = rand::thread_rng().gen_range(kanji_val_range.clone());
        let zen_app = zen_number * zen_base_app;

        let nanako_base_str = 15;
        let nanako_base_number = 7;
        let nanako_number = rand::thread_rng().gen_range(hiragana_val_range.clone());
        let nanako_str = ((nanako_number * nanako_base_str)/nanako_base_number) as i32;

        let zen = format!("{0}条禅", &num_to_kanji(zen_number));
        let nanako = format!("{0}条{1}こ", &rand_val_kanji(kanji_val_range.clone()), &num_to_hiragana(nanako_number));

        let zen_urlenc = urlencoding::encode(&zen);
        let nanako_urlenc = urlencoding::encode(&nanako);

        let tweet_url = format!("https://twitter.com/intent/tweet?text={0}%0A{1}%0A%0Ahttps%3A%2F%2Fas-is-prog.github.io%2Fn_jyo_n%2F",
                                       zen_urlenc,
                                       nanako_urlenc);

        html!{
            <div class="container">
            <div class="row">
                <div class="col card">
                    <div class="card-body">
                        <h3 class="card-title">{ "1d100条禅生成(1d100条いちdひゃくこにも対応)" }</h3>
                        <button class="btn btn-secondary" onclick={_ctx.link().callback(|_| UIMsg::Clicked)}>{ "リロール" }</button>
                        <br/>
                        <br/>
                        <a class="btn btn-primary" href={tweet_url} target="_blank" rel="noopener noreferrer">
                            { "ツイート" }
                        </a>
                    </div>
                </div>
            </div>

                <div class="row">
                    <div class="col card">
                        // <div class="card-img-top" style="height: 200px; width: 18rem; backglound-color: #99cc00;"/>
                        <div class="card-body">
                            <h4 class="card-title">{ zen }</h4>
                            <p class="card-text">{format!("今のAPPは{}です。", zen_app)}</p>
                        </div>
                    </div>
                </div>

                <div class="row">
                    <div class="col card">
                        // <div class="card-img-top" style="height: 200px; width: 18rem; backglound-color: #99cc00;"/>
                        <div class="card-body">
                            <h4 class="card-title">{ nanako }</h4>
                            <p class="card-text">{format!("今のSTRは{}です。", nanako_str)}</p>
                        </div>
                    </div>
                </div>

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