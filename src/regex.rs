use crate::ffi::*;
use std::cell::RefCell;
use std::sync::Arc;
use thread_local::ThreadLocal;

pub struct RegexBuilder;

impl RegexBuilder {
    pub fn new() -> Self {
        RegexBuilder {}
    }
    pub fn build(&self, pattern: &str) -> Result<Regex, String> {
        let options = 0;
        let ctx = CompileContext::new();
        let code = Code::new(pattern, options, ctx)?;
        Ok(Regex {
            pattern: pattern.to_string(),
            code: Arc::new(code),
            match_data: ThreadLocal::new(),
        })
    }
}

pub struct Regex {
    pattern: String,
    code: Arc<Code>,
    match_data: ThreadLocal<RefCell<MatchData>>,
}

impl Regex {
    pub fn new(pattern: &str) -> Result<Regex, String> {
        RegexBuilder::new().build(pattern)
    }
    pub fn find_iter<'r, 's>(&'r self, subject: &'s [u8]) -> Matches<'r, 's> {
        Matches {
            re: self,
            match_data: self.match_data(),
            subject,
            last_end: 0,
            last_match: None,
        }
    }

    fn match_data(&self) -> &RefCell<MatchData> {
        let create = || RefCell::new(MatchData::new(&self.code));
        self.match_data.get_or(create)
    }

    #[inline(always)]
    fn find_at_with_match_data<'s>(
        &self,
        match_data: &RefCell<MatchData>,
        subject: &'s [u8],
        start: usize,
    ) -> Result<Option<Match<'s>>, String> {
        let options = 0;
        let mut match_data = match_data.borrow_mut();
        if unsafe { !match_data.find(&&self.code, subject, start, options)? } {
            return Ok(None);
        }
        let ovector = match_data.ovector();
        let (s, e) = (ovector[0], ovector[1]);
        Ok(Some(Match::new(&subject[s..e], s, e)))
    }
}

pub struct Matches<'r, 's> {
    re: &'r Regex,
    match_data: &'r RefCell<MatchData>,
    subject: &'s [u8],
    last_end: usize,
    last_match: Option<usize>,
}

impl<'r, 's> Iterator for Matches<'r, 's> {
    type Item = Result<Match<'s>, String>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.last_end > self.subject.len() {
            return None;
        }
        let res = self
            .re
            .find_at_with_match_data(self.match_data, self.subject, self.last_end);
        let m = match res {
            Err(err) => return Some(Err(err)),
            Ok(None) => return None,
            Ok(Some(m)) => m,
        };
        if m.start() == m.end() {
            self.last_end = m.end() + 1;
            if Some(m.end()) == self.last_match {
                return self.next();
            }
        } else {
            self.last_end = m.end();
        }
        self.last_match = Some(m.end());
        Some(Ok(m))
    }
}

pub struct Match<'s> {
    subject: &'s [u8],
    start: usize,
    end: usize,
}

impl<'s> Match<'s> {
    #[inline(always)]
    pub fn start(&self) -> usize {
        self.start
    }
    #[inline(always)]
    pub fn end(&self) -> usize {
        self.end
    }
    pub fn new(subject: &'s [u8], start: usize, end: usize) -> Match<'s> {
        Match {
            subject,
            start,
            end,
        }
    }
}
