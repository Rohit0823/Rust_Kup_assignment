#[cfg(test)]
mod tests {
    use crate::book_library::Books;
    #[test]
    fn structure_book_check() {
        env_logger::init();
        let book_check_first = Books {
            accession_number: vec![01, 02],
            author: vec!["Robert Kiyosaki".to_string(), "Paulo Coelho".to_string()],
            book_title: vec!["Rich dad Poor dad".to_string(), "The alchemist".to_string()],
            flag: 2,
        };
        assert_eq!(book_check_first.display_book(), Ok(true));
    }

    #[test]
    fn structure_book_displayed_check_next() {
        let book_check_first = Books {
            accession_number: vec![],
            author: vec![],
            book_title: vec![],
            flag: 0,
        };
        assert_eq!(book_check_first.display_book(), Err(0));
    }

    #[test]
    pub fn add_new_book_check() {
        let mut book_check_first = Books {
            accession_number: vec![01],
            author: vec!["Paulo Coelho".to_string()],
            book_title: vec!["The alchemist".to_string()],
            flag: 1,
        };
        let book_check_second = Books {
            accession_number: vec![02],
            author: vec!["Robert Kiyosaki".to_string()],
            book_title: vec!["Rich dad Poor dad".to_string()],
            flag: 1,
        };
        assert_eq!(book_check_first.add_new_book(&book_check_second), Ok(true));
    }

    #[test]
    pub fn display_author_name_check() {
        let book_check_first = Books {
            accession_number: vec![01],
            author: vec!["Paulo Coelho".to_string()],
            book_title: vec!["The alchemist".to_string()],
            flag: 1,
        };
        assert_eq!(
            book_check_first.display_author_name("Paulo Coelho".to_string()),
            Ok(true)
        );
    }

    #[test]
    pub fn display_author_name_check_next() {
        let book_check_first = Books {
            accession_number: vec![01],
            author: vec!["Paulo Coelho".to_string()],
            book_title: vec!["The alchemist".to_string()],
            flag: 1,
        };
        assert_eq!(book_check_first.display_author_name("iMac".to_string()), Err(0));
    }

    #[test]
    fn display_book_title_check() {
        let book_check_first = Books {
            accession_number: vec![01],
            author: vec!["Paulo Coelho".to_string()],
            book_title: vec!["The alchemist".to_string()],
            flag: 1,
        };

        assert_eq!(
            book_check_first.display_particular_title("The alchemist".to_string()),
            Ok(1)
        );
    }

    #[test]
    fn display_book_title_check_next() {
        let book_check_first = Books {
            accession_number: vec![01],
            author: vec!["Paulo Coelho".to_string()],
            book_title: vec!["The alchemist".to_string()],
            flag: 1,
        };

        assert_eq!(
            book_check_first.display_particular_title("onePlus".parse().unwrap()),
            Err(0)
        );
    }

    #[test]
    fn total_book_success() {
        let books = Books {
            book_title: vec![
                "winner stand alone".to_string(),
                "The alchemist".to_string(),
            ],
            author: vec!["paulo".to_string(), "coelho".to_string()],
            accession_number: vec![1, 2],
            flag: 0,
        };
        let output = books.total_book();
        assert_eq!(output, Some(2))
    }
    #[test]
    fn total_book_failure() {
        let books = Books {
            book_title: vec![],
            author: vec![],
            accession_number: vec![],
            flag: 0,
        };
        let output = books.total_book();
        assert_eq!(output, None)
    }

    #[test]
    fn issue_book_check() {
        let mut book_check_first = Books {
            accession_number: vec![01, 02],
            author: vec!["Robert Kiyosaki".to_string(), "Paulo Coelho".to_string()],
            book_title: vec!["Rich dad Poor dad".to_string(), "The alchemist".to_string()],
            flag: 2,
        };

        assert_eq!(
            book_check_first.issue_book("The alchemist".parse().unwrap()),
            Some(1)
        );
    }
}