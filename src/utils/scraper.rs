use once_cell::sync::OnceCell;
use smartstring::{Compact, SmartString};
use tracing::{info, instrument};

/// required header for a valid http request.
/// update if you want, prolly won't matter
const DUMMY_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/85.0.4183.83 Safari/537.36/8mqQhSuL-09";

/// all fields for this struct are private, but its methods are public
#[derive(Debug)]
pub struct SteelCutter {
    page: Links,
    node_tree: OnceCell<NodeTree>,
}

type NodeTree = Vec<std::collections::BTreeMap<String, String>>;

impl SteelCutter {
    /// a new SteelCutter instance will be useless w/o calling the fetch method
    pub fn new(page: Links) -> Self {
        SteelCutter {
            page,
            node_tree: OnceCell::new(),
        }
    }

    /// this method uses a mutable borrow of self, and there can be only one of these per scope in Rust.
    /// plenty of resources explaining this online.
    /// TL;DR: create a new scope if you need to have two mutable references. i.e. :
    /// cutter.fetch()...
    /// {
    ///     // this is a separate scope
    ///     cutter.fetch()...
    /// }
    #[instrument]
    pub async fn fetch(&mut self) -> anyhow::Result<()> {
        let client = reqwest::ClientBuilder::new()
            .user_agent(DUMMY_USER_AGENT)
            .build()?;

        let response = client.get(&self.page.display().await).send().await;
        let body = response?.text().await?;

        let parser = match easy_scraper::Pattern::new(self.page.pattern().await) {
            Ok(pattern) => pattern,
            Err(e) => {
                info!("easy_scraper error: {:?}", e);
                anyhow::bail!("easy_scraper pattern failed to unwrap")
            }
        };
        let html_nodes = parser.matches(&body);
        if let Err(e) = self.node_tree.set(html_nodes) {
            info!("unable to set SteelCutter.node_tree value: {:?}", e);
            anyhow::bail!("unable to hydrate once_cell with html_nodes")
        }
        Ok(())
    }

    /// retrieve a single node value, returns only the first to match
    pub fn _get_node_val<'a, 'b>(&'a self, key: &'b str) -> Option<&'a str> {
        if let Some(tree) = self.node_tree.get() {
            tree.iter().fold(None, |none, node| {
                if let Some(value) = node.get(key) {
                    return Some(value);
                }
                none
            })
        } else {
            None
        }
    }

    /// retrieves all matching node values
    pub fn get_nodes_vec(&self, key: &str) -> Option<Vec<SmartString<Compact>>> {
        if let Some(tree) = self.node_tree.get() {
            let mut output = vec![];
            for wrapped_node in tree {
                if let Some(node) = wrapped_node.get(key) {
                    let node: SmartString<Compact> = node.into();
                    if !output.contains(&node) {
                        output.push(node);
                    }
                }
            }
            Some(output)
        } else {
            None
        }
    }
}

#[derive(Debug)]
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
            <p>{{about}}</p>
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

        let base_url = crate::CONFIG.lock().await.get_site_url().to_string();
        base_url
            + match self {
                About => "/about",
                Goals => "/goals",
                Shop => "/shop",
            }
    }
}
