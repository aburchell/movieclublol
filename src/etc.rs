use maud::{html, Markup};

pub fn header() -> Markup {
    html! {
        head {
            meta charset="UTF-8" {}
            meta name="viewport" content="width=device-width, initial-scale=1.0" {}
            meta http-equiv="X-UA-Compatible" content="ie=edge" {}
            title {"MOVIES"}
            link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bulma@0.9.4/css/bulma.min.css" {}
        }
    }
}
