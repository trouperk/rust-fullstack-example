use crate::entity::{
    form::article_editor::{Problem, ValidForm},
    Article, Slug, Viewer,
};
use crate::{coder::decoder, request};
use futures::prelude::*;
use seed::fetch::{Method, ResponseDataResult};
use serde::Deserialize;
use std::borrow::Cow;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RootDecoder {
    article: decoder::Article,
}

pub fn update<Ms: 'static>(
    viewer: Option<Viewer>,
    valid_form: &ValidForm,
    slug: &Slug,
    f: fn(Result<Article, Vec<Problem>>) -> Ms,
) -> impl Future<Item = Ms, Error = Ms> {
    request::new(&format!("articles/{}", slug.as_str()), viewer.as_ref())
        .method(Method::Put)
        .send_json(&valid_form.to_encoder())
        .fetch_json_data(move |data_result: ResponseDataResult<RootDecoder>| {
            f(data_result
                .map_err(request::fail_reason_into_problems)
                .and_then(move |root_decoder| {
                    root_decoder
                        .article
                        .try_into_article(viewer.map(Cow::Owned))
                        .map_err(|error| vec![Problem::new_server_error(error.into_inner())])
                }))
        })
}
