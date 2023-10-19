// Global solution 
extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let grapheme:Vec<&str> = UnicodeSegmentation::graphemes(input, true).collect();
    let reverse_grapheme:Vec<&str> = grapheme.into_iter().rev().collect();

    reverse_grapheme.concat()
}


/*  
* simple solution
pub fn reverse(input: &str) -> String{
    input.chars().rev().collect()
}
*/