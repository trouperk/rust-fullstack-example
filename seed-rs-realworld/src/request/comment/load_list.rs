use crate::entity::{Comment, ErrorMessage, Slug, Viewer};
use crate::{coder::decoder, logger, request};
use futures::prelude::*;
use seed::fetch::ResponseDataResult;
use serde::Deserialize;
use std::borrow::Cow;
use std::collections::VecDeque;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RootDecoder {
    comments: VecDeque<decoder::Comment>,
}

impl RootDecoder {
    fn into_comments(self, viewer: Option<&Viewer>) -> VecDeque<Comment> {
        self.comments
            .into_iter()
            .filter_map(|comment_decoder| {
                match comment_decoder.try_into_comment(viewer.map(Cow::Borrowed)) {
                    Ok(comment) => Some(comment),
                    Err(error) => {
                        logger::error(error);
                        None
                    }
                }
            })
            .collect()
    }
}

pub fn load_list<Ms: 'static>(
    viewer: Option<Viewer>,
    slug: &Slug,
    f: fn(Result<VecDeque<Comment>, Vec<ErrorMessage>>) -> Ms,
) -> impl Future<Item = Ms, Error = Ms> {
    request::new(
        &format!("articles/{}/comments", slug.as_str()),
        viewer.as_ref(),
    )
    .fetch_json_data(move |data_result: ResponseDataResult<RootDecoder>| {
        f(data_result
            .map(move |root_decoder| root_decoder.into_comments(viewer.as_ref()))
            .map_err(request::fail_reason_into_errors))
    })
}
