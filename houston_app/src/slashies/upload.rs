use std::fmt::Display;

use crate::prelude::*;

/// Uploads a file to an ephemeral message. Allows sharing if you are logged into multiple devices.
#[poise::command(slash_command)]
pub async fn upload(
    ctx: HContext<'_>,
    #[description = "The file to upload."]
    attachment: Attachment
) -> HResult {
    let description = format!(
        "**{}**\n> {}",
        attachment.filename,
        StorageSize(attachment.size)
    );

    let mut embed = CreateEmbed::new()
        .color(DEFAULT_EMBED_COLOR)
        .description(description);

    if attachment.dimensions().is_some() {
        embed = embed.thumbnail(attachment.proxy_url);
    }

    let components = CreateActionRow::Buttons(vec![
        CreateButton::new_link(attachment.url)
            .label("Download")
    ]);

    let reply = ctx.create_ephemeral_reply()
        .embed(embed)
        .components(vec![components]);

    ctx.send(reply).await?;
    Ok(())
}

struct StorageSize(u32);

impl Display for StorageSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const FACTOR: u32 = 1024;
        const KB: u32 = FACTOR;
        const MB: u32 = KB * FACTOR;
        const KB_LIMIT: u32 = MB - 1;

        match self.0 {
            s @ ..=KB_LIMIT => write!(f, "{:.1} KB", f64::from(s) / f64::from(KB)),
            s => write!(f, "{:.1} MB", f64::from(s) / f64::from(MB)),
        }
    }
}
