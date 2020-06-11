use model::ObjectType;
use db::{insert, select};

pub fn read() -> ObjectType {
    select()
}

pub fn write(data: &ObjectType) {
    insert(data)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
