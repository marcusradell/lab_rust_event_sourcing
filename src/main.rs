use uuid::Uuid;

mod product;

fn main() {
    let uuid = Uuid::new_v4();
    let id = uuid.to_hyphenated().to_string();

    let event = product::created::Event {
        name: "Teddybear".into(),
        id,
    };
    let product = product::created::fold(event);

    let event = product::name_changed::Event {
        name: "Nallebjörn".into(),
    };
    let product = product::name_changed::fold(product, event);

    println!("product: {:?}", product);
}
