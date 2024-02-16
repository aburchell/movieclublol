use maud::{html, Markup};
use sqlx::{postgres::Postgres, Pool};

pub fn panel_html() -> Markup {
    html! {
        iframe name="hacky" style="display:none;" {}
        form ."add-panel" ."box" name="addReviewForm" method="POST" action="/addReview" style="position: fixed; bottom: 10px; padding: 1em; margin: 1em;" target="hacky"{
            h2 {"Review a film!"}
            ."field" {
                label ."label" { "Title" }
                ."control" {
                    input ."input" type="text" name="title" autofill="false" required {}
                }
            }
            ."field" {
                label ."label" { "Director" }
                ."control" {
                    input ."input" type="text" name="director" autofill="false"{}
                } }
            ."field" {
                label ."label" { "Thoughts?" }
                ."control" {
                    input ."input" type="text" name="copy" autofill="false" required {}
                } }
            ."field" {
                ."control" {
                    button ."button" ."is-link" onclick="document.addReviewForm.submit(); setTimeout(() => document.addReviewForm.reset(), 10); console.log('Reset!')" { "Submit" } 
                }
            }
        }
    }
}
