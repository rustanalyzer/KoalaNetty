use std::io;

use crate::net::{client::Client, model::{FriendsRequest, GiftRequest}};

use crate::net::{
    model::{
        NickChangeRequest,
        NickColorChangeRequest,
        PolChangeRequest,
        TextInfoChangeRequest,
    },
};

pub struct Api<'a> {
    pub(crate) client: &'a mut Client,
}

impl<'a> Api<'a> {
    pub async fn friends(&mut self) -> io::Result<()> {
        self.client
            .send_json(
                0,
                &FriendsRequest {
                    cmd: "api",
                    method: "bot.friends",
                },
            )
            .await
    }

    pub async fn send_gift(&mut self, userid: i32, itemid: i32) -> io::Result<()> {
        self.client
            .send_json(
                0,
                &GiftRequest {
                    cmd: "api",
                    method: "bot.sendGift",
                    userid,
                    itemid,
                },
            )
            .await
    }

    pub async fn nick_change(&mut self, value: impl Into<String>) -> io::Result<()> {
        self.client
            .send_json(
                0,
                &NickChangeRequest {
                    cmd: "api",
                    method: "bot.nickChange",
                    v: value.into(),
                },
            )
            .await
    }

    pub async fn nick_color_change<S>(
        &mut self,
        colors: impl IntoIterator<Item = S>,
    ) -> io::Result<()>
    where
        S: AsRef<str>,
    {
        let value = colors
            .into_iter()
            .map(|s| s.as_ref().to_string())
            .collect::<Vec<_>>()
            .join(" ");

        self.client
            .send_json(
                0,
                &NickColorChangeRequest {
                    cmd: "api",
                    method: "bot.nickColorChange",
                    v: value,
                },
            )
            .await
    }

    pub async fn pol_change(&mut self, value: impl Into<String>) -> io::Result<()> {
        self.client
            .send_json(
                0,
                &PolChangeRequest {
                    cmd: "api",
                    method: "bot.polChange",
                    v: value.into(),
                },
            )
            .await
    }

    pub async fn text_info_change(&mut self, value: impl Into<String>) -> io::Result<()> {
        self.client
            .send_json(
                0,
                &TextInfoChangeRequest {
                    cmd: "api",
                    method: "bot.textInfoChange",
                    v: value.into(),
                },
            )
            .await
    }
}