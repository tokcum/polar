use model::ObjectType;
use appl::pass;

fn main() {
    let ot = ObjectType::new();

    pass(&ot);

    println!("ObjectType: {:?}", ot);
}
