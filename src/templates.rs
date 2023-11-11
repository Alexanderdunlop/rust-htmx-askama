use askama::Template;

#[derive(Template)]
#[template(path = "timer.html")]
pub struct TimerTemplate {
    pub oob: bool,
    pub ms: String
}


#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {}