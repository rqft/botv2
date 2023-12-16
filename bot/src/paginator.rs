use std::{fmt::Display, marker::PhantomData, sync::Arc};

use crate::common::Context;

use poise::{
    serenity_prelude::{
        self as serenity, AttachmentType, CacheHttp, CreateComponents, CreateEmbed,
        CreateInteractionResponseData, CreateMessage, MessageComponentInteraction, ReactionType,
    },
    CreateReply, ReplyHandle,
};

pub const MAX_PAGE: usize = usize::MAX;
pub const MIN_PAGE: usize = 1usize;

pub enum Inter<'a, 'b> {
    Start(&'a mut CreateReply<'b>),
    Update(&'a mut CreateInteractionResponseData<'b>),
}

impl<'a, 'b> Inter<'a, 'b> {
    pub fn content<T>(&mut self, content: T) -> &mut Self
    where
        T: Display,
    {
        match self {
            Self::Start(b) => drop(b.content(format!("{content}"))),
            Self::Update(b) => drop(b.content(format!("{content}"))),
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

    pub fn attachments<'c: 'a + 'b>(&mut self, a: Vec<AttachmentType<'c>>) -> &mut Self {
        match self {
            Self::Start(b) => a.into_iter().for_each(|x| drop(b.attachment(x))),
            Self::Update(b) => drop(b.files(a)),
        }

        self
    }
}

pub trait GetPage<T> = Fn(usize) -> T;
pub trait OnPage<T> = (for<'b, 'c> Fn(usize, T, Inter<'b, 'c>) -> Inter<'b, 'c>);

#[derive(Clone)]
pub struct Paginator<'a, T, P, Q>
where
    Q: GetPage<T>,
    P: OnPage<T>,
{
    pub context: Context<'a>,
    pub options: PaginatorOptions<T, P, Q>,
    pub stopped: bool,
    pub page: usize,
}

#[derive(Clone)]
pub struct PaginatorOptions<T, P, Q>
where
    Q: GetPage<T>,
    P: OnPage<T>,
{
    pub get_page: Q,
    pub on_page: P,
    pub is_ephemeral: Option<bool>,
    pub expires: u64,
    pub buttons: Option<Buttons>,
    pub targets: Option<Vec<serenity::UserId>>,
    pub page_limit: usize,
    pub none: PhantomData<T>,
}

#[derive(Clone, Debug)]
pub struct Buttons {
    // pub custom: Option<FakeButton>,
    pub next: Option<FakeButton>,
    pub next_large: Option<FakeButton>,
    pub previous: Option<FakeButton>,
    pub previous_large: Option<FakeButton>,
    // pub shuffle: Option<FakeButton>,
    pub stop: Option<FakeButton>,
}

#[derive(Clone, Default, Debug)]
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
            // next_large: Some(FakeButton::label(">>")),
            next_large: None,
            previous: Some(FakeButton::label("<")),
            // previous_large: Some(FakeButton::label("<<")),
            previous_large: None,
            // shuffle: None,
            stop: Some(FakeButton::label("X")),
        }
    }
}

impl<'a, T, P, Q> Paginator<'a, T, P, Q>
where
    Q: GetPage<T>,
    P: OnPage<T>,
{
    pub fn new(context: Context<'a>, options: PaginatorOptions<T, P, Q>) -> Self {
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
            let buttons = self.options.buttons.as_ref().cloned().unwrap_or_default();
            if let Some(prev_large) = buttons.previous_large {
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

            if let Some(prev) = buttons.previous {
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

            if let Some(next) = buttons.next {
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

            if let Some(next_large) = buttons.next_large {
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

            if let Some(stop) = buttons.stop {
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

        // dbg!(&value);

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
            .send(|x| {
                let data = (self.options.get_page)(self.page);
                let v = (self.options.on_page)(self.page, data, Inter::Start(x));

                match v {
                    Inter::Start(v) => {
                        if self.options.page_limit != 1 {
                            v.components(|x| self.components(x))
                        } else {
                            v
                        }
                    }
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
            // dbg!(&press.data.custom_id);
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
                        let data = (self.options.get_page)(self.page);
                        let v = (self.options.on_page)(self.page, data, Inter::Update(x));

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
        value: &ReplyHandle<'_>,
    ) {
        value.delete(self.context).await.unwrap();
    }
}
