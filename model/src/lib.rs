pub use self::objecttype::ObjectType;

mod id;
mod objecttype;
mod rev;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
