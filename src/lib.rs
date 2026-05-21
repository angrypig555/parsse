
pub struct rss_object {
    pub title: String,
    pub link: String,
    pub description: String,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
