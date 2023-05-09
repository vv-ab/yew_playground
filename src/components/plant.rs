use std::borrow::Cow;
use gloo_timers::callback::Timeout;
use log::{error, info};
use reqwasm::Error;
use reqwasm::http::Request;
use yew::{Html, html};
use yew::prelude::*;
use yew_playground_model::{Plant, PlantWateringHistory};

const wateringcan_a: &'static str = "wateringcan.png";
const wateringcan_b: &'static str = "wateringcan3.gif";

#[derive(Properties, PartialEq)]
pub struct Props {
    pub plant: Plant
}

#[function_component(PlantView)]
pub fn plant_view(props: &Props) -> Html {
    let name = Clone::clone(&props.plant.name);
    let watering_icon = use_state(|| wateringcan_a);

    let watering_action = {
        let watering_icon = Clone::clone(&watering_icon);
        let name = Clone::clone(&name);
        move |_: MouseEvent| {
            let name = Clone::clone(&name);
            watering_icon.set(wateringcan_b);
            {
                let watering_icon = Clone::clone(&watering_icon.clone());
                Timeout::new(3000, move || watering_icon.set(wateringcan_a))
                    .forget();
            }
            wasm_bindgen_futures::spawn_local(async move {
                let plants_endpoint = format!("/api/plant/{}/watering", name);
                let result = Request::post(&plants_endpoint).send().await;

                match result {
                    Ok(response) => {
                        info!("Watered {}", name);
                    }
                    Err(e) => {
                        error!("Failed to water plant: {}", name);
                    },
                }
            });
        }
    };
    {
        let name = Clone::clone(&name);
        use_effect(move || {
            wasm_bindgen_futures::spawn_local(async move {
                let plants_endpoint = format!("/api/plant/{}/waterlevel", name);
                let result = Request::get(&plants_endpoint).send().await;

                match result {
                    Ok(response) => {
                        let json: Result<PlantWateringHistory, _> = response.json().await;
                        match json {
                            Ok(watering_history) => {
                                info!("history: {:?}", watering_history);
                            }
                            Err(_) => {
                                error!("Failed to parse history");
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to fetch watering history: {}", name);
                    },
                }
            });
        });
    }

    html! {
        <div class="columns">
            <div class="column is-two-thirds">
                <div class="subtitle is-3">{"Name: "}{name}</div>
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
            </div>
        </div>
    }
}