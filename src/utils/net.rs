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
    link_urls.insert(About, "https://www.steelcutkawaii.com/about");
    link_urls.insert(Donate, "https://secure.squarespace.com/checkout/donate?donatePageId=5f25897026a28453d038f64a&ss_cvr=6f6f279b-f7c6-4278-9d23-c2c8ceebd537%7C1603010061453%7C1603010061453%7C1603010061453%7C1");
    link_urls.insert(Email, "mailto: xxspidderxx@gmail.com");
    link_urls.insert(Goals, "https://www.steelcutkawaii.com/goals");
    link_urls.insert(Patreon, "https://www.patreon.com/LittleMissClub");
    link_urls.insert(Shop, "https://www.steelcutkawaii.com/shop");
    link_urls.insert(Stream, "https://www.chaturbate.com/xx_spidder_xx");
    link_urls.insert(Twitter, "https://twitter.com/LittleMissMuf18");
    link_urls.insert(Venmo, "https://venmo.com/LilMissMuffet");
    link_urls.insert(YouTube, "https://www.youtube.com/c/XxSpidderxX");

    link_urls
});

#[derive(Hash, PartialEq, Eq)]
pub enum Links {
    About,
    Donate,
    Email,
    Goals,
    Patreon,
    Shop,
    Stream,
    Twitter,
    YouTube,
    Venmo,
}

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
        use Links::*;
        match self {
            About => {
                r#"
            <div data-block-type="2">
            <p>{{about1}}</p>
            <p>{{about2}}
            </div>
                "#
            }
            Goals => {
                r#"
		    <li>
			    <p>{{goal}}</p>
		    </li>"#
            }
            Shop => {
                r#"
            <div data-controller="ProductListImageLoader">
		    <a href="{{shop_url}}" aria-label="{{item_name}}"></a>
	        </div>
                "#
            }
            _ => "",
        }
    }
}
