//use yew::services::ConsoleService;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::services::ConsoleService;
use yewtil::future::LinkFuture;

use pouch::{Error, InfoResponse, DB};

struct Model {
    link: ComponentLink<Self>,
    value: i64,
    db_info: pouch::InfoResponse,
}

enum Msg {
    AddOne,
    FetchDBInfo,
    FetchDBInfoDone(pouch::InfoResponse),
    FetchDBInfoFailed,
}

async fn fetch_db_info() -> Result<InfoResponse, Error> {
    ConsoleService::info("Pouch Yew example: Fetching database info");
    let db = DB::new("examples_yew");
    db.info().await
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
            db_info: InfoResponse {
                db_name: String::from("unknown"),
                adapter: String::from("unknown"),
                idb_attachment_format: String::from("unknown"),
                doc_count: 0,
                update_seq: 0,
                auto_compaction: false,
            },
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                true
            }
            Msg::FetchDBInfo => {
                let future = async {
                    match fetch_db_info().await {
                        Ok(info) => Msg::FetchDBInfoDone(info),
                        Err(_) => Msg::FetchDBInfoFailed,
                    }
                };
                self.link.send_future(future);
                false
            }
            Msg::FetchDBInfoDone(info) => {
                ConsoleService::info("Pouch Yew example: Fetching database info done");
                self.db_info = info;
                true
            }
            Msg::FetchDBInfoFailed => {
                ConsoleService::error("Pouch Yew example: Fetching database info failed");
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <p><b>{ format!("{} (v{})", "Yew & Pouch", pouch::version()) }</b></p>
                <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                <p>{ self.value }</p>
                <button onclick=self.link.callback(|_| Msg::FetchDBInfo)>{ "Get Database Info" }</button>
                <p><i>{ format!("{:?}", self.db_info) }</i></p>
                </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
