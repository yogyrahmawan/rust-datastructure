

mod single_linked_list;
mod double_linked_list;

#[cfg(test)]
mod tests{
    use crate::*;

    #[test]
    fn better_transaction_log_append() {
        let mut transaction_log = double_linked_list::BetterTransactionLog::new_empty();
        assert_eq!(transaction_log.length, 0);
        transaction_log.append("INSERT INTO mytable VALUES (1,2,3)".to_owned());
        transaction_log.append("INSERT INTO mytable VALUES (2,3,4)".to_owned());
        transaction_log.append("INSERT INTO mytable VALUES (3,4,5)".to_owned());
        assert_eq!(transaction_log.length, 3);
        assert_eq!(
            transaction_log.pop(),
            Some("INSERT INTO mytable VALUES (1,2,3)".to_owned())
        );
        assert_eq!(
            transaction_log.pop(),
            Some("INSERT INTO mytable VALUES (2,3,4)".to_owned())
        );
        assert_eq!(
            transaction_log.pop(),
            Some("INSERT INTO mytable VALUES (3,4,5)".to_owned())
        );
        assert_eq!(transaction_log.pop(), None);
    }

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