use model::ObjectType;
use db::push;

pub fn pass(data: &ObjectType) {
    push(data)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
