use yew::prelude::*;
use web_sys::HtmlInputElement;
use rand::Rng;

fn main() {
    yew::start_app::<CounterComponent>();
}

fn to_digit(num: u8) -> char{
    if num <= 9{
        return (num+48) as char
    }
    return (num-10+65) as char
}

fn value(c: char) -> u8{
    if c>='0' && c<='9'{
        return (c as u8) - 48
    }
    return (c as u8)-55
}

fn to_decimal(num: &str, base: u8) -> Option<u128>{
    let base_converted: u128 = base as u128;
    let mut res: u128 = 0;
    let mut pow: u128= 1;
    for c in num.chars().rev(){
        let digit: u8 = value(c);
        if digit>=base{
            return Option::None
        }
        res += (digit as u128) * pow;
        pow *= base_converted;
    }
    return Option::Some(res)
}

fn from_decimal(num: u128, tobase: u8) -> String{
    let mut num = num.clone();
    let mut res: String = String::new();
    let base_converted: u128 = tobase as u128;
    while num>0{
        res.push(to_digit((num%base_converted) as u8));
        num/=base_converted;
    }
    res.chars().rev().collect::<String>()
}

enum Msg{
    Input(String, u32),
    Enter,
    Swap,
    TogglePractice,
    None,
}

struct CounterComponent{
    input: String,
    res: String,
    from_base: u8,
    to_base: u8,
    practice_mode: bool,
    checked_ans: bool,
    practice_ans: String,
}

impl Component for CounterComponent{
    type Message = Msg;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self{
        Self{input: String::new(), res: String::new(), from_base: 2, to_base: 10, practice_mode: false, practice_ans: String::new(), checked_ans: true}
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool{
        match msg{
            Msg::Input(content, field) => {
                match field{
                    1 => self.from_base = content.parse::<u8>().unwrap_or(0),
                    2 => self.to_base = content.parse::<u8>().unwrap_or(0),
                    3 => self.res=content.to_uppercase(),
                    _ => self.input=content.to_uppercase()
                }
                true
            }
            Msg::Enter => {
                if self.practice_mode{
                    if self.checked_ans{
                        self.checked_ans=false;
                        return true
                    }
                    self.checked_ans = true;
                    self.input=from_decimal(rand::thread_rng().gen_range(0..10000), self.from_base)
                }
                let mut dec: u128 = self.input.parse::<u128>().unwrap_or(0);
                if self.from_base != 10{
                    match to_decimal(&self.input, self.from_base){
                        Some(num) => dec=num,
                        None => {self.res=String::from("Invalid input"); return true}
                    }
                }
                if self.to_base != 10{
                    self.res = from_decimal(dec, self.to_base).to_string();
                    return true
                }
                if self.practice_mode{
                    self.practice_ans=dec.to_string();
                }else{
                    self.res=dec.to_string();
                }
                true
            }
            Msg::Swap =>{
                let temp: u8 = self.from_base;
                self.from_base=self.to_base;
                self.to_base=temp;
                let temp2: String = self.input.clone();
                self.input=self.res.clone();
                self.res=temp2;
                true
            }
            Msg::TogglePractice => {
                if self.practice_mode {self.practice_mode=false}
                else {self.practice_mode=true}
                true
            }
            Msg::None =>{
                //PreviousInput::create(yew::);
                false
            }
        }
    }
    fn view(&self, _ctx: &Context<Self>) -> Html{
        let link = _ctx.link();
        html! {
            <div class = "content">
                <h1>{"Base Converter!"}</h1>
                //<div class="comparison center-block">
                //    <input type="integer" id="from" name="from" oninput={link.callback(|event: InputEvent| {let input: HtmlInputElement = event.target_unchecked_into(); Msg::Input(input.value(), 1)})} onkeypress={link.callback(|key:KeyboardEvent| {if key.char_code()==13 {Msg::None} else{Msg::None}})}/>
                //    <input type="integer" id="to" name="to" oninput={link.callback(|event: InputEvent| {let input: HtmlInputElement = event.target_unchecked_into(); Msg::Input(input.value(), 2)})} onkeypress={link.callback(|key:KeyboardEvent| {if key.char_code()==13 {Msg::None} else{Msg::None}})}/>
                //</div>
                <div class="field3 center-block flex">
                    <label class="togglebutton center-block">
                        <input type="checkbox" onclick={link.callback(|_| Msg::TogglePractice)} checked={self.practice_mode}/>
                        <span class="tbutton">{"Practice mode"}</span>
                    </label>
                </div>
                <div class="field center-block flex">
                    <input class="basefield" type="text" id="label1" name="label1" value={self.from_base.to_string()} onfocus={link.callback(|e: FocusEvent| {let i: HtmlInputElement = e.target_unchecked_into(); i.select(); Msg::None})} oninput={link.callback(|event: InputEvent| {let input: HtmlInputElement = event.target_unchecked_into(); Msg::Input(input.value(),1)})} />
                    if self.practice_mode{<input class="numfield" type="text" id="num_output" name="num_output" value={self.input.clone()} readonly=true onfocus={link.callback(|e: FocusEvent| {let i: HtmlInputElement = e.target_unchecked_into(); i.select(); Msg::None})}/>}
                    else{<input class="numfield" type="text" id="num_input" name="num_input" value={self.input.clone()} oninput={link.callback(|event: InputEvent| {let input: HtmlInputElement = event.target_unchecked_into(); Msg::Input(input.value(),0)})} onkeypress={link.callback(|key:KeyboardEvent| {if key.char_code()==13 {Msg::Enter} else{Msg::None}})}/>}
                </div>
                <div class="field2 center-block flex">
                    <button class="basefield" onclick={link.callback(|_| Msg::Swap)}>{"swap"}</button>
                    if self.practice_mode && !self.checked_ans{
                        if self.practice_ans==self.res{<input class="numfield correct" type="text" id="practice_ans" name="practice_ans" value={"Correct!"} readonly=true/>}
                        else{<input class="numfield incorrect" type="text" id="practice_ans" name="practice_ans" value={self.practice_ans.clone()} readonly=true/>}
                    }
                </div>
                <div class="field center-block flex">
                    <input class="basefield" type="text" id="label2" name="label2" value={self.to_base.to_string()} onfocus={link.callback(|e: FocusEvent| {let i: HtmlInputElement = e.target_unchecked_into(); i.select(); Msg::None})} oninput={link.callback(|event: InputEvent| {let input: HtmlInputElement = event.target_unchecked_into(); Msg::Input(input.value(),2)})} />
                    if self.practice_mode {<input class="numfield" type="text" id="num_input" name="num_input" value={self.res.clone()} oninput={link.callback(|event: InputEvent| {let input: HtmlInputElement = event.target_unchecked_into(); Msg::Input(input.value(),3)})} onkeypress={link.callback(|key:KeyboardEvent| {if key.char_code()==13 {Msg::Enter} else{Msg::None}})}/>}
                    else{<input class="numfield" type="text" id="num_output" name="num_output" value={self.res.clone()} readonly=true onfocus={link.callback(|e: FocusEvent| {let i: HtmlInputElement = e.target_unchecked_into(); i.select(); Msg::None})}/>}
                </div>
                <div class="field center-block flex">
                    if self.practice_mode {
                        if self.checked_ans {<button class="center-block buttonfield" onclick={link.callback(|_| Msg::Enter)}>{"Check"}</button>}
                        else {<button class="center-block buttonfield" onclick={link.callback(|_| Msg::Enter)}>{"New"}</button>}
                    }
                    else {<button class="center-block buttonfield" onclick={link.callback(|_| Msg::Enter)}>{"Solve"}</button>}
                </div>
            </div>
        }
    }
}