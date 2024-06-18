use std::collections::HashSet;

pub fn check(candidate: &str) -> bool
{
    //define a bunding that is a hash set of type char
    //The mut key word opts into mutibility for the variable letters
    let mut letters: HashSet<char> = HashSet::new();

    //iterate over the characters in the string
    for character in candidate.chars()
    {
        //validate the character is a letter
        if character.is_ascii_alphabetic()
        {
            //transform the character to lower case
            let lower_letter = character.to_ascii_lowercase();

            //chack if the letter is equal to a letter that already has been processed
            if letters.contains(&lower_letter)
            {
                return false;
            }

            letters.insert(lower_letter);
        }
    }

    return true;
}
