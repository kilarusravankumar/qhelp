use crate::msg::Msg;
use reqwest;
use serde::{Deserialize, Serialize};
use std::vec;

#[derive(Serialize, Debug)]
pub struct ChatReq {
    pub model: String,
    pub stream: bool,
    pub messages: vec::Vec<Msg>,
}

#[derive(Debug, Deserialize)]
pub struct ChatRes {
    pub message: Msg,
}

pub fn query_llm(
    url: String,
    model: String,
    msgs: vec::Vec<Msg>,
) -> Result<ChatRes, reqwest::Error> {
    let req_body = ChatReq {
        model: model,
        stream: false,
        messages: msgs,
    };
    let llm_client = reqwest::blocking::Client::new();
    // dbg!(&llm_client);
    let res: ChatRes = llm_client.post(url).json(&req_body).send()?.json()?;

    // println!(" LLm response is {}", res.message.content);
    dbg!(&res);

    Ok(res)
}

