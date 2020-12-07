use crate::{prelude::*, Links};

/*
*   scraping functions (SteelCutter) require more setup/know-how
*   if you really wanna make one, dm me
*/

#[command]
async fn shop(ctx: &Context, msg: &Message) -> CommandResult {
    let mut cutter = SteelCutter::new(Links::Shop);
    if cutter.fetch().await.is_ok() {
        if let Some(name_nodes) = cutter.get_nodes_vec("item_name") {
            if let Some(url_nodes) = cutter.get_nodes_vec("shop_url") {
                let zipped = name_nodes.iter().zip(url_nodes).fold(
                    String::new(),
                    |mut message, (name, path)| {
                        message.push_str(&name);
                        message.push_str("\n");
                        let mut url = String::from("https://www.steelcutkawaii.com");
                        url.push_str(&path);
                        message.push_str(&url);
                        message.push_str("\n\n");
                        message
                    },
                );
                announce(ctx, msg, zipped).await?;
            }
        }
    }
    Ok(())
}

#[command]
async fn about(ctx: &Context, msg: &Message) -> CommandResult {
    let mut cutter = SteelCutter::new(Links::About);
    if cutter.fetch().await.is_ok() {
        let mut message = String::new();
        if let Some(about1) = cutter.get_node_val("about1") {
            message.push_str(&about1);
        }
        if let Some(about2) = cutter.get_node_val("about2") {
            message.push_str(&about2);
        }
        announce(ctx, msg, message).await?;
    }
    Ok(())
}

#[command]
async fn donate(ctx: &Context, msg: &Message) -> CommandResult {
    announce(ctx, msg, Links::Donate).await
}

#[command]
async fn goal(ctx: &Context, msg: &Message) -> CommandResult {
    let mut cutter = SteelCutter::new(Links::Goals);
    if cutter.fetch().await.is_ok() {
        if let Some(goal_nodes) = cutter.get_nodes_vec("goal") {
            let mut rand_int = rand::random::<usize>();
            let num_goals = goal_nodes.len();
            if rand_int > num_goals {
                rand_int = rand_int % num_goals;
            }
            let rand_goal = &goal_nodes[rand_int];
            announce(ctx, msg, rand_goal).await?;
        }
    }
    Ok(())
}

#[command]
async fn goals(ctx: &Context, msg: &Message) -> CommandResult {
    announce(ctx, msg, Links::Goals).await
}

#[command]
async fn stream(ctx: &Context, msg: &Message) -> CommandResult {
    announce(ctx, msg, Links::Stream).await
}
