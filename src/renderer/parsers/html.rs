use nom::{
    bytes::complete::{tag, take_until},
    character::complete::char,
    complete::tag,
    sequence::tuple,
    IResult, Parser,
};
use tracing::info;

use crate::{renderer::RenderError, PoppetError, PoppetResult};

#[derive(Debug)]
pub struct Element {
    pub name: String,
}

impl Element {
    pub fn new(s: &str) -> Self {
        Self {
            name: String::from(s),
        }
    }
}

pub fn parse(html: &str) -> PoppetResult<Element> {
    //
    let (input, (el)) = (html_opening_tag)
        .parse(html)
        .map_err(|e| RenderError::HTMLParseError(e.to_string()))?;

    Ok(el)
}

fn html_opening_tag(input: &str) -> IResult<&str, Element> {
    let el = tuple((char('<'), take_until(">"), char('>')))(input)?;
    //info!("parsed: {:?}", el.1);
    Ok((input, Element::new(el.1 .1)))
}

fn html_tag_content(input: &str) -> IResult<&str, &str> {
    let el = tuple((tag("</"), take_until(">"), char('>')))(input)?;
    //info!("parsed: {:?}", el.1);
    Ok((input, Element::new(el.1 .1)))
}

fn html_closing_tag(input: &str) -> IResult<&str, Element> {
    let el = tuple((tag("</"), take_until(">"), char('>')))(input)?;
    //info!("parsed: {:?}", el.1);
    Ok((input, Element::new(el.1 .1)))
}
