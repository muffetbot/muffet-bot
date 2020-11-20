use once_cell::sync::Lazy;
use std::collections::HashMap;

/* only requirements for adding new URLS:
*     - add a new field in Links
*     - insert a hash here with a valid URL
*
*   template hash creation:
*       link_urls.insert(NewLinksField, "https://www.valid.url.com/");
*/
static URLS: Lazy<HashMap<Links, &str>> = Lazy::new(|| {
    let mut link_urls = HashMap::new();
    use Links::*;
    link_urls.insert(Contact, "https://www.steelcutkawaii.com/contact");
    link_urls.insert(Donate, "https://secure.squarespace.com/checkout/donate?donatePageId=5f25897026a28453d038f64a&ss_cvr=6f6f279b-f7c6-4278-9d23-c2c8ceebd537%7C1603010061453%7C1603010061453%7C1603010061453%7C1");
    link_urls.insert(Email, "mailto: xxspidderxx@gmail.com");
    link_urls.insert(Goals, "https://www.steelcutkawaii.com/goals");
    link_urls.insert(Patreon, "https://www.patreon.com/LittleMissClub");
    link_urls.insert(Poetry, "https://www.steelcutkawaii.com/shop-1/zines");
    link_urls.insert(Stream, "https://www.chaturbate.com/xx_spidder_xx");
    link_urls.insert(Twitter, "https://twitter.com/LittleMissMuf18");
    link_urls.insert(Venmo, "https://venmo.com/LilMissMuffet");
    link_urls.insert(
        YouTube,
        "https://www.youtube.com/channel/UCz6dg88uZ0nHib3gjCAUw-w",
    );

    link_urls
});

#[derive(Hash, PartialEq)]
pub enum Links {
    Contact,
    Donate,
    Email,
    Goals,
    Patreon,
    Poetry,
    Stream,
    Twitter,
    YouTube,
    Venmo,
}

impl Eq for Links {}

impl AsRef<str> for Links {
    fn as_ref<'a>(&'a self) -> &'a str {
        URLS.get(self)
            .unwrap_or(&"you must create a hash for this Links field")
    }
}

impl Links {
    /// this method is used by the scraper.
    /// see https://docs.rs/easy-scraper/0.2.0/easy_scraper/ for documentation
    pub fn pattern<'a>(&'a self) -> &'a str {
        match self {
            Links::Contact => {
                r#"<div id="block-77e2f05ca8cd8a53543b"><div class="sqs-block-content"><h3>{{title}}</h3><p>{{meat}}</p></div></div>"#
            }
            Links::Goals => {
                r#"
		    <li>
			    <p>{{entry}}</p>
		    </li>"#
            }
            _ => "",
        }
    }
}
