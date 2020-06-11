pub use self::objecttype::ObjectType;

mod objecttype;
mod id;
mod rev;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
