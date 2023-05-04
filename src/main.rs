use yew::prelude::*;
use gloo_timers;
use gloo_timers::callback::Timeout;
use gloo_file::*;
use gloo_net::http::Request;
use yew::platform::spawn_local;
use yew_router::Routable;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/hello-server")]
    HelloServer,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Hello Frontend" }</h1> },
        Route::HelloServer => html! { <HelloServer /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    let watering_icon = use_state(|| "wateringcan.png" );
    let watering_action = {
        let watering_icon = Clone::clone(&watering_icon);
        move |_: MouseEvent| {
            let value = "wateringcan3.gif";
            watering_icon.set(value);

            {
                let watering_icon = Clone::clone(&watering_icon.clone());
                Timeout::new(3000, move || watering_icon.set("wateringcan.png"))
                    .forget();
            }
        }
    };

    html! {
            <div class="container hero is-fluid is-fullheight">

                <div class="title is-1">
                    <h1 id={ "projectName" }>
                        <span class={ "blackName" }>{ "T" }</span>
                        <span class={ "redName" }>{ "RUST" }</span>
                        <span class={ "blackName" }>{ "Y GARDENER" }</span>
                    </h1>
                </div>
                <div class="columns">
                    <div class="column is-two-thirds">
                        <div class="subtitle is-3">{"Beschreibung:"}</div>
                        <div class="tile is-ancestor">
                            <div class="tile is-parent">
                                <article class="tile is-child box">
                                    <p class="title is-4">{"Wasserstand"}</p>
                                </article>
                            </div>
                            <div class="tile is-parent">
                                <article class="tile is-child box">
                                    <p class="title is-4">{"Helligkeit"}</p>
                                </article>
                            </div>
                            <div class="tile is-parent">
                                <article class="tile is-child box">
                                    <p class="title is-4">{"Gießen"}</p>
                                    <p class="subtitle is-6">{ "Drücke auf das Bild, wenn du die Pflanze gegossen hast, um die Daten zu speichern:" }</p>
                                    <img onclick={watering_action} src={*watering_icon} alt="watering" title="watering can" width="100" height="100"/>
                                </article>
                            </div>
                        </div>
                        <div class="tile is-ancestor">
                            <div class="tile is-parent">
                                <article class="tile is-child box">
                                    <p class="title is-4">{"History"}</p>
                                </article>
                            </div>
                            <div class="tile is-parent">
                                <article class="tile is-child box">
                                    <p class="title is-4">{"optional"}</p>
                                </article>
                            </div>
                        </div>
                    </div>
                    <div class="column">
                        <div class="tile is-ancestor">
                            <div class="tile is-parent">
                                <article class="tile is-child box">
                                    <img src="plant1.png" alt="a plant" />
                                </article>
                            </div>
                        </div>


                        <div class="tile is-ancestor">
                            <div class="tile is-parent">
                                <article class="tile p-2 is-child box">
                                    <div class="select is-medium">
                                        <select>
                                            <option>{ "plant 1" }</option>
                                            <option>{ "plant 2" }</option>
                                        </select>
                                    </div>

                                </article>
                                <div class="box p-2">
                                    { "uploading files" }
                                </div>
                            </div>
                        </div>
                    </div>
                </div>

                <footer class="footer mt-auto">
                    <div class="content">
                        <a href="https://media.tenor.com/JHePvU1xhFgAAAAM/pump-crypto.gif">{"Hilfe"}</a>
                        <br/>
                        <a href="https://images.wagwalkingweb.com/media/daily_wag/behavior_guides/hero/1629934048.5675797/he-gong-kbuycu1swik-unsplash.jpg">{"Privatsphäre"}</a>
                        <br/>
                        <a href="https://media.tenor.com/YGWxkk8hm7UAAAAd/cat-telephone-cat.gif">{"Kontakt"}</a>
                        <br/>
                        <a href="https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcRAfL6JExBpyNxNEIpTcxhYVySM3yOmopdNpUqnL8_eIgXLvtXChBfBvoDrs7u8_E3nhBs&usqp=CAU">{"Bedingungen"}</a>
                        <br/>
                    </div>
                </footer>
            </div>
    }
}

#[function_component(HelloServer)]
fn hello_server() -> Html {
    let data = use_state(|| None);

    // Request `/api/hello` once
    {
        let data = data.clone();
        use_effect(move || {
            if data.is_none() {
                spawn_local(async move {
                    let resp = Request::get("/api/hello").send().await.unwrap();
                    let result = {
                        if !resp.ok() {
                            Err(format!(
                                "Error fetching data {} ({})",
                                resp.status(),
                                resp.status_text()
                            ))
                        } else {
                            resp.text().await.map_err(|err| err.to_string())
                        }
                    };
                    data.set(Some(result));
                });
            }

            || {}
        });
    }

    match data.as_ref() {
        None => {
            html! {
                <div>{"No server response"}</div>
            }
        }
        Some(Ok(data)) => {
            html! {
                <div>{"Got server response: "}{data}</div>
            }
        }
        Some(Err(err)) => {
            html! {
                <div>{"Error requesting data from server: "}{err}</div>
            }
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}