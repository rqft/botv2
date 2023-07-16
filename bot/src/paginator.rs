use std::sync::Arc;

use ::serenity::{
    builder::{CreateComponents, CreateEmbed, CreateInteractionResponseData, CreateMessage},
    http::CacheHttp,
};
use poise::serenity_prelude::{self as serenity, MessageComponentInteraction, ReactionType};
use wa::model::Pod;

use crate::{common::Context, embed_preset::user};

pub async fn paginate_wa(ctx: Context<'_>, pages: Vec<Pod>) -> Result<(), serenity::Error> {
    // Define some unique identifiers for the navigation buttons
    let ctx_id = ctx.id();
    let prev_button_id = format!("{}prev", ctx.id());
    let next_button_id = format!("{}next", ctx.id());
    let gone_button_id = format!("{}gone", ctx.id());

    // Send the embed with the first page as content
    let mut current_page = 0;
    ctx.send(|b| {
        b.embed(|b| {
            let x = user(ctx, b)
                .title(pages[current_page].title.clone())
                .footer(|x| x.text(format!("Page {}/{}", current_page, pages.len())));

            if let Some(v) = pages[current_page].subpods.get(0) {
                if let Some(i) = &v.img {
                    x.image(i.src.clone());
                }
            }

            x.fields(
                pages[current_page]
                    .subpods
                    .iter()
                    .filter(|x| x.plaintext.is_some())
                    .map(|x| (x.title.clone(), x.plaintext.clone().unwrap(), true)),
            )
        })
        .components(|b| {
            b.create_action_row(|b| {
                b.create_button(|b| b.custom_id(&prev_button_id).label('<'))
                    .create_button(|b| b.custom_id(&next_button_id).label('>'))
                    .create_button(|b| {
                        b.custom_id(&gone_button_id)
                            .label("X")
                            .style(poise::serenity_prelude::ButtonStyle::Danger)
                    })
            })
        })
    })
    .await?;

    // Loop through incoming interactions with the navigation buttons
    while let Some(press) = poise::serenity_prelude::CollectComponentInteraction::new(ctx)
        // We defined our button IDs to start with `ctx_id`. If they don't, some other command's
        // button was pressed
        .filter(move |press| press.data.custom_id.starts_with(&ctx_id.to_string()))
        // Timeout when no navigation button has been pressed for 24 hours
        .timeout(std::time::Duration::from_secs(3600 * 24))
        .await
    {
        if press.user.id != ctx.author().id {
            continue;
        }
        // Depending on which button was pressed, go to next or previous page
        if press.data.custom_id == next_button_id {
            current_page += 1;
            if current_page >= pages.len() {
                current_page = 0;
            }
        } else if press.data.custom_id == prev_button_id {
            current_page = current_page.checked_sub(1).unwrap_or(pages.len() - 1);
        } else if press.data.custom_id == gone_button_id {
            press
                .delete_original_interaction_response(ctx.http())
                .await?;
        } else {
            // This is an unrelated button interaction
            continue;
        }

        // Update the message with the new page contents
        press
            .create_interaction_response(ctx, |b| {
                b.kind(poise::serenity_prelude::InteractionResponseType::UpdateMessage)
                    .interaction_response_data(|b| {
                        b.embed(|b| {
                            let x = user(ctx, b)
                                .title(pages[current_page].title.clone())
                                .footer(|x| {
                                    x.text(format!("Page {}/{}", current_page, pages.len()))
                                });

                            if let Some(v) = pages[current_page].subpods.get(0) {
                                if let Some(i) = &v.img {
                                    x.image(i.src.clone());
                                }
                            }

                            x.fields(
                                pages[current_page]
                                    .subpods
                                    .iter()
                                    .filter(|x| x.plaintext.is_some())
                                    .map(|x| (x.title.clone(), x.plaintext.clone().unwrap(), true)),
                            )
                        })
                    })
            })
            .await?;
    }

    Ok(())
}

pub const MAX_PAGE: usize = usize::MAX;
pub const MIN_PAGE: usize = 1usize;

pub enum Inter<'a, 'b> {
    Start(&'a mut CreateMessage<'b>),
    Update(&'a mut CreateInteractionResponseData<'b>),
}

impl<'a, 'b> Inter<'a, 'b> {
    pub fn content<T>(&mut self, content: T) -> &mut Self
    where
        T: ToString,
    {
        match self {
            Self::Start(b) => drop(b.content(content)),
            Self::Update(b) => drop(b.content(content)),
        };

        self
    }

    pub fn embed(&mut self, f: impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed) -> &mut Self {
        match self {
            Self::Start(b) => drop(b.embed(f)),
            Self::Update(b) => drop(b.embed(f)),
        }
        self
    }
}

pub trait OnPage = (for<'b, 'c> Fn(usize, Inter<'b, 'c>) -> Inter<'b, 'c>);
pub trait OnStop = Fn();

#[derive(Clone)]
pub struct Paginator<'a, P, S>
where
    P: OnPage,
    S: OnStop,
{
    pub context: Context<'a>,
    pub options: PaginatorOptions<'a, P, S>,
    pub stopped: bool,
    pub page: usize,
}

#[derive(Clone)]
pub struct PaginatorOptions<'a, P, S>
where
    P: OnPage,
    S: OnStop,
{
    pub on_page: &'a P,
    pub is_ephemeral: Option<bool>,
    pub expires: u64,
    pub buttons: Option<Buttons>,
    pub targets: Option<Vec<serenity::UserId>>,
    pub page_limit: usize,
    pub on_stop: Option<&'a S>,
}

#[derive(Clone)]
pub struct Buttons {
    // pub custom: Option<FakeButton>,
    pub next: Option<FakeButton>,
    pub next_large: Option<FakeButton>,
    pub previous: Option<FakeButton>,
    pub previous_large: Option<FakeButton>,
    // pub shuffle: Option<FakeButton>,
    pub stop: Option<FakeButton>,
}

#[derive(Clone, Default)]
pub struct FakeButton {
    label: Option<String>,
    emoji: Option<ReactionType>,
}

impl FakeButton {
    pub fn label(label: impl Into<String>) -> Self {
        Self {
            label: Some(label.into()),
            emoji: None,
        }
    }

    pub fn emoji(emoji: impl Into<ReactionType>) -> Self {
        Self {
            label: None,
            emoji: Some(emoji.into()),
        }
    }
}

impl Default for Buttons {
    fn default() -> Self {
        Self {
            // custom: Some(CreateButton::default().label("?")),
            // custom: None,
            next: Some(FakeButton::label(">")),
            next_large: Some(FakeButton::label(">>")),
            previous: Some(FakeButton::label("<")),
            previous_large: Some(FakeButton::label("<<")),
            // shuffle: None,
            stop: Some(FakeButton::label("X")),
        }
    }
}

impl<'a, P, S> Paginator<'a, P, S>
where
    P: OnPage,
    S: OnStop,
{
    pub fn new(context: Context<'a>, options: PaginatorOptions<'a, P, S>) -> Self {
        Self {
            context,
            options,
            stopped: false,
            page: MIN_PAGE,
        }
    }

    pub fn previous_large_id(&self) -> String {
        format!("previous_large_{}", self.context.id())
    }

    pub fn previous_id(&self) -> String {
        format!("previous_{}", self.context.id())
    }

    pub fn next_id(&self) -> String {
        format!("next_{}", self.context.id())
    }

    pub fn next_large_id(&self) -> String {
        format!("next_large_{}", self.context.id())
    }

    pub fn shuffle_id(&self) -> String {
        format!("shuffle_{}", self.context.id())
    }

    pub fn custom_id(&self) -> String {
        format!("custom_{}", self.context.id())
    }

    pub fn stop_id(&self) -> String {
        format!("stop_{}", self.context.id())
    }

    pub fn components<'b>(&self, value: &'b mut CreateComponents) -> &'b mut CreateComponents {
        if self.stopped || self.options.page_limit == 1 {
            return value;
        }

        println!("test");

        value.create_action_row(|x| {
            if let Some(prev_large) = self
                .options
                .buttons
                .as_ref()
                .cloned()
                .and_then(|x| x.previous_large)
            {
                x.create_button(|x| {
                    let y = x
                        .custom_id(self.previous_large_id())
                        .disabled(self.page == MIN_PAGE);

                    if let Some(l) = prev_large.label {
                        y.label(l);
                    }
                    if let Some(e) = prev_large.emoji {
                        y.emoji(e);
                    }

                    y
                });
            }

            if let Some(prev) = self
                .options
                .buttons
                .as_ref()
                .cloned()
                .and_then(|x| x.previous)
            {
                x.create_button(|x| {
                    let y = x
                        .custom_id(self.previous_id())
                        .disabled(self.page == MIN_PAGE);
                    if let Some(l) = prev.label {
                        y.label(l);
                    }
                    if let Some(e) = prev.emoji {
                        y.emoji(e);
                    }

                    y
                });
            }

            if let Some(next) = self.options.buttons.as_ref().cloned().and_then(|x| x.next) {
                x.create_button(|x| {
                    let y = x
                        .custom_id(self.next_id())
                        .disabled(self.page == self.options.page_limit);
                    if let Some(l) = next.label {
                        y.label(l);
                    }
                    if let Some(e) = next.emoji {
                        y.emoji(e);
                    }

                    y
                });
            }

            if let Some(next_large) = self
                .options
                .buttons
                .as_ref()
                .cloned()
                .and_then(|x| x.next_large)
            {
                x.create_button(|x| {
                    let y = x
                        .custom_id(self.next_large_id())
                        .disabled(self.page == self.options.page_limit);
                    if let Some(l) = next_large.label {
                        y.label(l);
                    }
                    if let Some(e) = next_large.emoji {
                        y.emoji(e);
                    }

                    y
                });
            }

            // if let Some(mut shuffle) = self.options.buttons.as_ref().cloned().and_then(|x| x.shuffle) {
            //     x.add_button(
            //         shuffle
            //             .custom_id(format!("shuffle_{}", self.context.id()))
            //             .clone(),
            //     );
            // }

            // if let Some(mut custom) = self.options.buttons.as_ref().cloned().and_then(|x| x.custom) {
            //     x.add_button(
            //         custom
            //             .custom_id(format!("custom_{}", self.context.id()))
            //             .disabled(self.page == self.options.page_limit)
            //             .clone(),
            //     );
            // }

            if let Some(stop) = self.options.buttons.as_ref().cloned().and_then(|x| x.stop) {
                x.create_button(|x| {
                    let y = x
                        .custom_id(self.stop_id())
                        .style(serenity::ButtonStyle::Danger);

                    if let Some(l) = stop.label {
                        y.label(l);
                    }
                    if let Some(e) = stop.emoji {
                        y.emoji(e);
                    }

                    y
                });
            }

            x
        });

        dbg!(&value);

        value
    }

    pub fn channel_id(&self) -> serenity::ChannelId {
        self.context.channel_id()
    }

    pub fn id(&self) -> u64 {
        match self.context {
            poise::Context::Prefix(p) => p.msg.id.0,
            poise::Context::Application(a) => a.interaction.id().0,
        }
    }

    pub async fn start(mut self) {
        let value = self
            .context
            .channel_id()
            .send_message(self.context.http(), |x| {
                let v = (self.options.on_page)(self.page, Inter::Start(x));
                match v {
                    Inter::Start(v) => v.components(|x| self.components(x)),
                    _ => unreachable!(),
                }
            })
            .await
            .unwrap();

        let expires = self.options.expires;
        let ctx_id = self.context.id();

        println!("test {ctx_id}");

        while let Some(press) =
            poise::serenity_prelude::CollectComponentInteraction::new(self.context)
                // We defined our button IDs to start with `ctx_id`. If they don't, some other command's
                // button was pressed
                .filter(move |press| press.data.custom_id.ends_with(&ctx_id.to_string()))
                // Timeout when no navigation button has been pressed for 24 hours
                .timeout(std::time::Duration::from_millis(expires))
                .await
        {
            dbg!(&press.data.custom_id);
            if !self
                .options
                .targets
                .clone()
                .unwrap_or(vec![self.context.author().id])
                .contains(&press.user.id)
                && !(self
                    .context
                    .framework()
                    .options
                    .owners
                    .contains(&press.user.id))
            {
                continue; // not your target
            }

            if press.data.custom_id == self.next_id() {
                self.next(press).await;
            } else if press.data.custom_id == self.previous_id() {
                self.previous(press).await;
            } else if press.data.custom_id == self.next_large_id() {
                self.next_large(press).await;
            } else if press.data.custom_id == self.previous_large_id() {
                self.previous_large(press).await;
            } else if press.data.custom_id == self.stop_id() {
                self.stop(press, &value).await;
                return;
            }
        }
    }

    pub async fn update(&self, press: Arc<MessageComponentInteraction>) {
        press
            .create_interaction_response(self.context.http().clone(), |x| {
                x.kind(serenity::InteractionResponseType::UpdateMessage)
                    .interaction_response_data(|x| {
                        x.content("")
                            .set_embeds(vec![])
                            .files(std::iter::empty::<serenity::AttachmentType>());
                        let v = (self.options.on_page)(self.page, Inter::Update(x));

                        if let Inter::Update(v) = v {
                            v.components(|x| self.components(x))
                        } else {
                            unreachable!()
                        }
                    })
            })
            .await
            .unwrap();
    }

    pub async fn next(&mut self, press: Arc<MessageComponentInteraction>) {
        self.page += 1;
        self.update(press).await;
    }

    pub async fn previous(&mut self, press: Arc<MessageComponentInteraction>) {
        self.page -= 1;
        self.update(press).await;
    }

    pub async fn next_large(&mut self, press: Arc<MessageComponentInteraction>) {
        self.page = self.options.page_limit;
        self.update(press).await;
    }

    pub async fn previous_large(&mut self, press: Arc<MessageComponentInteraction>) {
        self.page = MIN_PAGE;
        self.update(press).await;
    }

    pub async fn stop(
        &mut self,
        _press: Arc<MessageComponentInteraction>,
        value: &serenity::Message,
    ) {
        value.delete(self.context.http()).await.unwrap();
    }
}
