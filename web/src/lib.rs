use seed::{prelude::*, *};

pub struct Model {
    ot: String,
}

// ------ ------
//    Update
// ------ ------
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        ot: String::new(),
    }
}

enum Msg {
    NewObjectType,
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::NewObjectType => {
            model.ot = String::new();
        }
    }
}

fn view(model: &Model) -> impl IntoNodes<Msg> {
    div![format!("It Works: {:?}", model.ot)]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
