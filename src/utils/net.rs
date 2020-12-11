pub enum Links {
    About,
    Goals,
    Shop,
}

impl Links {
    /// this method is used by the scraper.
    /// see https://docs.rs/easy-scraper/0.2.0/easy_scraper/ for documentation
    pub async fn pattern<'a>(&'a self) -> &'a str {
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
        }
    }

    pub async fn display(&self) -> String {
        use Links::*;

        let mut base_url = (&crate::CONFIG.lock().await).site_url.to_string();
        match self {
            About => base_url.push_str("/about"),
            Goals => base_url.push_str("/goals"),
            Shop => base_url.push_str("/shop"),
        }

        base_url
    }
}
