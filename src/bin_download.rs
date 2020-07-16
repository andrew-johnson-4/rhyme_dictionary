/* Dictionaries, located at https://www.subarctic.org/wiktionary_pronunciation_[lang].txt
   are compiled from Wiktionary which gives them all a CreativeCommons Attribution
   Share-Alike License (https://creativecommons.org/licenses/by-sa/3.0/). The same license
   is applied to any additions or modifications made by maintainers and contributors of
   the rhyme_dictionary project and by the hosting provider (subarctic.org).
*/

use error_chain::error_chain;
use std::io::copy;
use std::fs::File;

error_chain! {
     foreign_links {
         Io(std::io::Error);
         HttpRequest(reqwest::Error);
     }
}

#[tokio::main]
async fn main() -> Result<()> {
    let target = "https://www.subarctic.org/logos/wiktionary_pronunciation_eng.txt";
    let local = "data/wiktionary_pronunciation_eng.txt";
    let response = reqwest::get(target).await?;

    let mut dest = {
        println!("file to download: '{}' to '{}'", target, local);
        File::create(local)?
    };
    let content =  response.text().await?;
    copy(&mut content.as_bytes(), &mut dest)?;
    Ok(())
}
