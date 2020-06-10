pub use self::datatype::DataType;

mod datatype;
mod id;
mod rev;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
