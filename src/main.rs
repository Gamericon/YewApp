use yew::prelude::*;

enum Msg {
    Addone
}

struct CounterComponent{
    count: i64,

}

impl counter for CounterComponent {
     type Message = Msg;
     type Properties = ();

     fn create(_ctx: &Context<Self>) -> Self {
        Self { count : 0 }
     }
     fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Addone => {
                self.count += 1;
                true
            }
        }
     }

     fn view(&self, _ctx: &Context<self>) -> Html {
        html! {
            <div classes="container">
              <p>{ self.count } </p>
              <button onclick={link.callback(|_| Msg::Addone)}>{ "+1" }</button>
              </div>
        }
     }
}
fn main() {
    
    yew::start_app::<CounterComponent>();

}
