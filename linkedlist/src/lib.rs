

mod single_linked_list;

#[cfg(test)]
mod tests{
    use crate::*;
    
    #[test]
    fn transaction_log_append(){
        let mut transaction_log = single_linked_list::TransactionLog::new_empty();

        assert_eq!(transaction_log.length, 0);
        transaction_log.append("UPDATE CUSTOMER SET NAME = 'A'".to_owned());
        transaction_log.append("UPDATE CUSTOMER SET NAME = 'B'".to_owned());
        transaction_log.append("UPDATE CUSTOMER SET NAME = 'C'".to_owned());

        assert_eq!(transaction_log.length, 3);
        assert_eq!(
            transaction_log.pop(), 
            Some("UPDATE CUSTOMER SET NAME = 'A'".to_owned())
        );
        assert_eq!(
            transaction_log.pop(), 
            Some("UPDATE CUSTOMER SET NAME = 'B'".to_owned())
        );
        assert_eq!(
            transaction_log.pop(), 
            Some("UPDATE CUSTOMER SET NAME = 'C'".to_owned())
        );
        assert_eq!(transaction_log.pop(), None);   
    }

    
}