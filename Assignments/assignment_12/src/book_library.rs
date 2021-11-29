use log::*;

    /// Struct: It contains the data of the book
    ///
    /// #fields
    ///
    /// accession_number: It is store Vec<i32> type
    ///
    /// author: It is store Vec<String> type
    ///
    /// book_title: It is store Vec<String> type
    ///
    /// flag: flag value tell about the status
pub struct Books {
    pub accession_number: Vec<i32>,
    pub author: Vec<String>,
    pub book_title: Vec<String>,
    pub flag: i32,
}

impl Books {
    /// display_book: this function is used to add the book details in the data
    ///
    /// #Arguments
    ///
    /// there is no argument here
    ///
    /// #Return
    ///
    /// Return type Result<bool, i32>

    pub fn display_book(&self) -> Result<bool, i32> {
        if self.accession_number.is_empty() {
            error!("nothing to do");
            return Err(0);
        }
        for element in 0..self.accession_number.len() {
            info!(
                "accession_number:{},Author: {} , book_title: {} , ",
                self.accession_number[element],
                self.author[element],
                self.book_title[element]
            );
        }
        Ok(true)
    }
    /// add_new_book: It is used to add the new book
    ///
    /// #Arguments
    ///
    /// book_name: A book_name is the Structure of an Object
    ///
    /// #Return
    ///
    /// Return type Result<bool, i32>

    pub fn add_new_book(&mut self, book_name: &Books) -> Result<bool, i32> {
        if self
            .accession_number
            .contains(&book_name.accession_number[0])
        {
            error!("Book is present");
            return Err(0);
        }
        self.accession_number.push(book_name.accession_number[0]);

        let dir = book_name.author[0].clone();
        self.author.push(dir);
        let dir = book_name.book_title[0].clone();
        self.book_title.push(dir);
        self.flag += book_name.flag;

        Ok(true)
    }
    /// display_author_name: It gives the book information related to the author
    ///
    /// #Arguments
    ///
    /// author_check: it is String type which store author name
    ///
    /// #Return
    ///
    /// Return type Result<bool, i32>

    pub fn display_author_name(&self, author_check: String) -> Result<bool, i32> {
        if !self.author.contains(&author_check) {
            error!("data unavailable");
            return Err(0);
        }
        for element in 0..self.author.len() {
            if author_check == self.author[element] {
                info!(
                    "accession_number:{},Author: {} , book_title: {} , ",
                    self.accession_number[element],
                    self.author[element],
                    self.book_title[element]
                )
            }
        }
        Ok(true)
    }
    /// display_particular_title: It gives the book information related to the title
    ///
    /// #Arguments
    ///
    /// title_check: it is String type which store title_name
    ///
    /// #Return
    ///
    /// Return type Result<i32, i32>

    pub fn display_particular_title(&self, title_check: String) -> Result<i32, i32> {
        if !self.book_title.contains(&title_check) {
            error!("data is not related");
            return Err(0);
        }
        let mut book_count = 0;
        for element in 0..self.book_title.len() {
            if title_check == self.book_title[element] {
                book_count += 1;
            }
        }
        if book_count == 0 {
            error!("Please give number");
        } else {
            info!("book counting {}", book_count)
        }
        Ok(1)
    }
    /// total_book: It represent the total number of books
    ///
    /// #Arguments
    ///
    /// &self: it is self type parameter
    ///
    /// #Return
    ///
    /// Return type Option<usize>

    pub fn total_book(&self) -> Option<usize> {
        match !self.accession_number.is_empty() {
            true => Some(self.accession_number.len()),
            false => None,
        }
    }
    /// issue_book: It can used to issue the book and update the data
    ///
    /// #Arguments
    ///
    /// book_name: it is a string type which tell about the issued book
    ///
    /// #Return
    ///
    /// return type is Option<i32>

    pub fn issue_book(&mut self, book_name: String) -> Option<i32> {
        let book_contain = self
            .book_title
            .iter()
            .position(|value| *value == book_name)
            .unwrap();
        {
            self.book_title.remove(book_contain);
            self.author.remove(book_contain);
            self.accession_number.remove(book_contain);
            self.flag -= 1;
            info!("book issued SuccessFully");
            Some(1)
        }
    }
}