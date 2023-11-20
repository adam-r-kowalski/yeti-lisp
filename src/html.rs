extern crate alloc;

use crate::effect::{error, Effect};
use crate::evaluate_expressions;
use crate::expression::Environment;
use crate::extract;
use crate::Expression::{self, NativeFunction};
use alloc::boxed::Box;
use alloc::format;
use alloc::string::{String, ToString};
use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use im::{ordmap, vector, OrdMap, Vector};
use markup5ever_rcdom::{Handle, NodeData, RcDom};

type Result<T> = core::result::Result<T, Effect>;

fn self_closing(tag: &str) -> bool {
    match tag {
        "area" => true,
        "base" => true,
        "br" => true,
        "col" => true,
        "embed" => true,
        "hr" => true,
        "img" => true,
        "input" => true,
        "link" => true,
        "meta" => true,
        "param" => true,
        "source" => true,
        "track" => true,
        "wbr" => true,
        _ => false,
    }
}

fn style_tag(style_map: OrdMap<Expression, Expression>, string: &mut String) -> Result<()> {
    string.push_str("<style>");
    for (k, v) in style_map {
        let selector = extract::keyword(k.clone())?;
        let rules_map = extract::map(v.clone())?;
        string.push_str(&selector[1..]);
        string.push_str(" { ");
        for (rule_key, rule_value) in rules_map {
            let rule_property = extract::keyword(rule_key.clone())?;
            let rule_val_str = extract::string(rule_value)?;
            string.push_str(&rule_property[1..]);
            string.push_str(": ");
            string.push_str(&rule_val_str);
            string.push_str("; ");
        }
        string.push_str("}");
    }
    string.push_str("</style>");
    Ok(())
}

fn is_array_of_array(expr: Expression) -> Option<Vector<Expression>> {
    if let Expression::Array(a) = expr {
        if let Some(Expression::Array(_)) = a.get(0) {
            return Some(a);
        }
    }
    None
}

fn build_children_string(children: Vector<Expression>, string: &mut String) -> Result<()> {
    for child in children {
        if let Some(children) = is_array_of_array(child.clone()) {
            build_children_string(children, string)?;
        } else {
            build_string(child, string)?;
        }
    }
    Ok(())
}

pub fn build_string(expr: Expression, string: &mut String) -> Result<()> {
    match expr {
        Expression::Array(a) => {
            let keyword = extract::keyword(a[0].clone())?;
            let keyword = &keyword[1..];
            if keyword == "style" {
                let style_map = extract::map(a[1].clone())?;
                return style_tag(style_map, string);
            }
            string.push('<');
            string.push_str(keyword);
            if a.len() > 1 {
                if let Expression::Map(m) = &a[1] {
                    for (k, v) in m.iter() {
                        let k = extract::keyword(k.clone())?;
                        string.push(' ');
                        string.push_str(&k[1..]);
                        string.push_str("=\"");
                        let s = extract::string(v.clone())?;
                        string.push_str(&s);
                        string.push('"');
                    }
                    if self_closing(keyword) {
                        string.push_str(" />");
                        Ok(())
                    } else {
                        string.push('>');
                        build_children_string(a.clone().split_off(2), string)?;
                        string.push_str("</");
                        string.push_str(keyword);
                        string.push('>');
                        Ok(())
                    }
                } else if self_closing(keyword) {
                    string.push_str(" />");
                    Ok(())
                } else {
                    string.push('>');
                    build_children_string(a.clone().split_off(1), string)?;
                    string.push_str("</");
                    string.push_str(keyword);
                    string.push('>');
                    Ok(())
                }
            } else if self_closing(keyword) {
                string.push_str(" />");
                Ok(())
            } else {
                string.push_str("></");
                string.push_str(keyword);
                string.push('>');
                Ok(())
            }
        }
        Expression::String(s) => {
            string.push_str(&s);
            Ok(())
        }
        Expression::Integer(i) => {
            string.push_str(&format!("{}", i));
            Ok(())
        }
        _ => Err(error("Expected keyword")),
    }
}

async fn to_string(
    env: Environment,
    args: Vector<Expression>,
) -> Result<(Environment, Expression)> {
    let (env, args) = evaluate_expressions(env, args).await?;
    let mut string = String::new();
    build_string(args[0].clone(), &mut string)?;
    Ok((env, Expression::String(string)))
}

fn dom_to_expression(handle: &Handle) -> Expression {
    match &handle.data {
        NodeData::Doctype {
            name,
            public_id,
            system_id,
        } => Expression::Array(vector![
            Expression::Keyword(":doctype".to_string()),
            Expression::Map(ordmap! {
                Expression::Keyword(":name".to_string()) => Expression::String(name.to_string()),
                Expression::Keyword(":public_id".to_string()) => Expression::String(public_id.to_string()),
                Expression::Keyword(":system_id".to_string()) => Expression::String(system_id.to_string())
            })
        ]),
        NodeData::Document => {
            let children = handle.children.borrow();
            let expressions: Vector<Expression> = children.iter().map(dom_to_expression).collect();
            if expressions.len() == 1 {
                expressions[0].clone()
            } else {
                Expression::Array(expressions)
            }
        }
        NodeData::Element { name, attrs, .. } => {
            let tag = Expression::Keyword(format!(":{}", name.local));
            let mut result = vector![tag];
            let attributes = attrs.borrow();
            if !attributes.is_empty() {
                let mut map = ordmap! {};
                for attr in attributes.iter() {
                    let key = Expression::Keyword(format!(":{}", attr.name.local));
                    let value = Expression::String(attr.value.to_string());
                    map.insert(key, value);
                }
                result.push_back(Expression::Map(map));
            }
            let children = handle.children.borrow();
            for child in children.iter() {
                let child = dom_to_expression(child);
                result.push_back(child);
            }
            Expression::Array(result)
        }
        NodeData::Text { contents } => Expression::String(contents.borrow().to_string()),
        e => panic!("unexpected {:?}", e),
    }
}

pub fn html_string_to_expression(html: &str) -> Expression {
    let parser = parse_document(RcDom::default(), Default::default());
    let dom = parser.one(html);
    dom_to_expression(&dom.document)
}

async fn from_string(
    env: Environment,
    args: Vector<Expression>,
) -> Result<(Environment, Expression)> {
    let (env, args) = evaluate_expressions(env, args).await?;
    let html = extract::string(args[0].clone())?;
    let expression = html_string_to_expression(&html);
    Ok((env, expression))
}

pub fn environment() -> Environment {
    ordmap! {
        "*name*".to_string() => Expression::String("html".to_string()),
        "to-string".to_string() => NativeFunction(
            |env, args| Box::pin(to_string(env, args))
        ),
        "from-string".to_string() => NativeFunction(
            |env, args| Box::pin(from_string(env, args))
        )
    }
}
