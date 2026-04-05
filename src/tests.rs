#[cfg(test)]
mod tests {
    use crate::backend::DemutBackend;

    #[test]
    fn test_to_string() {
        let backend = DemutBackend::new(
            String::from("http://"), 
            String::from("127.0.0.1"), 
            8081
        ); 
        
        let backend2 = DemutBackend::new(
            String::from("tcp://"),
            String::from("192.168.1.15"),
            8091
        );

        let result = backend.to_string();
        let result2 = backend2.to_string();

        assert_eq!(result, "http://127.0.0.1:8081");
        assert_eq!(result2, "tcp://192.168.1.15:8091");
    }
}