// Copyright 2014-2017 The html5ever Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Types used within the tree builder code.  Not exported to users.

use {LocalName, QualName};
use tokenizer::Tag;
use tokenizer::states::RawKind;
use tokenizer::TagKind;

use tendril::{SendTendril, StrTendril};
use tendril::fmt::UTF8;

pub use self::InsertionMode::*;
pub use self::SplitStatus::*;
pub use self::Token::*;
pub use self::ProcessResult::*;
pub use self::FormatEntry::*;
pub use self::InsertionPoint::*;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum InsertionMode {
    Initial,
    BeforeHtml,
    BeforeHead,
    InHead,
    InHeadNoscript,
    AfterHead,
    InBody,
    Text,
    InTable,
    InTableText,
    InCaption,
    InColumnGroup,
    InTableBody,
    InRow,
    InCell,
    InSelect,
    InSelectInTable,
    InTemplate,
    AfterBody,
    InFrameset,
    AfterFrameset,
    AfterAfterBody,
    AfterAfterFrameset,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum SplitStatus {
    NotSplit,
    Whitespace,
    NotWhitespace,
}

/// A subset/refinement of `tokenizer::Token`.  Everything else is handled
/// specially at the beginning of `process_token`.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Token {
    TagToken(Tag),
    CommentToken(StrTendril),
    CharacterTokens(SplitStatus, StrTendril),
    NullCharacterToken,
    EOFToken,
}

pub enum ProcessResult<Handle> {
    Done,
    DoneAckSelfClosing,
    SplitWhitespace(StrTendril),
    Reprocess(InsertionMode, Token),
    ReprocessForeign(Token),
    Script(Handle),
    ToPlaintext,
    ToRawData(RawKind),
}

#[derive(Clone)]
pub enum SendableFormatEntry<Handle> {
    Element {
        handle: Handle,
        tag_kind: TagKind,
        tag_name: LocalName,
        tag_self_closing: bool,
        tag_attrs: Vec<(QualName, SendTendril<UTF8>)>,
    },
    Marker
}

pub enum FormatEntry<Handle> {
    Element(Handle, Tag),
    Marker,
}

pub enum InsertionPoint<Handle> {
    /// Insert as last child in this parent.
    LastChild(Handle),
    /// Insert before this following sibling.
    BeforeSibling(Handle),
    /// Insertion point is decided based on existence of element's parent node.
    TableFosterParenting { element: Handle, prev_element: Handle },
}
