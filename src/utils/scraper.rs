use crate::Links;
use once_cell::sync::OnceCell;
use smartstring::{Compact, SmartString};

/// required header for a valid http request.
/// update if you want, prolly won't matter
const DUMMY_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/85.0.4183.83 Safari/537.36/8mqQhSuL-09";

/// all fields for this struct are private, but its methods are public
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
    pub async fn fetch(&mut self) -> anyhow::Result<()> {
        let client = reqwest::ClientBuilder::new()
            .user_agent(DUMMY_USER_AGENT)
            .build()?;

        let response = client.get(self.page.as_ref()).send().await;
        let body = response?.text().await?;

        let parser = match easy_scraper::Pattern::new(self.page.pattern()) {
            Ok(pattern) => pattern,
            Err(e) => anyhow::bail!(e),
        };
        let html_nodes = parser.matches(&body);
        if self.node_tree.set(html_nodes).is_err() {
            anyhow::bail!("unable to set SteelCutter.node_tree value")
        }
        Ok(())
    }

    // retrieve a single node value, returns only the first to match
    pub fn get_node_val<'a, 'b>(&'a self, key: &'b str) -> Option<&'a str> {
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

    // retrieves all matching node values
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
